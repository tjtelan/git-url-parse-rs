use anyhow::Result;
use log::debug;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use strum_macros::{EnumString, EnumVariantNames};
use url::Url;

/// Supported uri schemes for parsing
#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum Protocol {
    /// Represents No url scheme
    Unspecified,
    /// Represents `file://` url scheme
    File,
    /// Represents `http://` url scheme
    Http,
    /// Represents `https://` url scheme
    Https,
    /// Represents `ssh://` url scheme
    Ssh,
    /// Represents `git://` url scheme
    Git,
    /// Represents `git+ssh://` url scheme
    #[strum(serialize = "git+ssh")]
    GitSsh,
}

/// GitUrl represents an input url `href` that is a url used by git
/// Internally during parsing the url is sanitized and uses the `url` crate to perform
/// the majority of the parsing effort, and with some extra handling to expose
/// metadata used my many git hosting services
#[derive(Debug, PartialEq)]
pub struct GitUrl {
    /// The input url
    pub href: String,
    /// The fully qualified domain name (FQDN) or IP of the repo
    pub host: Option<String>,
    /// The name of the repo
    pub name: String,
    /// The owner/account/project name
    pub owner: Option<String>,
    /// The organization name. Supported by Azure DevOps
    pub organization: Option<String>,
    /// The full name of the repo, formatted as "owner/name"
    pub fullname: String,
    /// The git url protocol
    pub protocol: Protocol,
    /// The authentication user
    pub user: Option<String>,
    /// The oauth token (could appear in the https urls)
    pub token: Option<String>,
    /// The non-conventional port where git service is hosted
    pub port: Option<u16>,
    /// The path to repo w/ respect to user + hostname
    pub path: String,
    /// Indicate if url uses the .git suffix
    pub git_suffix: bool,
}

impl fmt::Display for GitUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.href)
    }
}

impl Default for GitUrl {
    fn default() -> Self {
        GitUrl {
            href: "".to_string(),
            host: None,
            name: "".to_string(),
            owner: None,
            organization: None,
            fullname: "".to_string(),
            protocol: Protocol::Unspecified,
            user: None,
            token: None,
            port: None,
            path: "".to_string(),
            git_suffix: true,
        }
    }
}

impl GitUrl {
    /// Returns a new `GitUrl` with provided `url` set as `href`
    pub fn new(url: &str) -> GitUrl {
        GitUrl {
            href: url.to_string(),
            ..Default::default()
        }
    }

    /// Returns a `Result<GitUrl>` after normalizing and parsing `url` for metadata
    pub fn parse(url: &str) -> Result<GitUrl> {
        // Normalize the url so we can use Url crate to process ssh urls
        let normalized = normalize_url(url).expect("Url normalization failed");

        // Some pre-processing for paths
        let protocol = Protocol::from_str(normalized.scheme())
            .expect(&format!("Protocol unsupported: {:?}", normalized.scheme()));

        // Normalized ssh urls can always have their first '/' removed
        let urlpath = match &protocol {
            Protocol::Ssh => {
                // At the moment, we're relying on url::Url's parse() behavior to not duplicate
                // the leading '/' when we normalize
                normalized.path()[1..].to_string()
            }
            _ => normalized.path().to_string(),
        };

        let git_suffix_check = &urlpath.ends_with(".git");

        // Parse through path for name,owner,organization
        // Support organizations for Azure Devops
        debug!("The urlpath: {:?}", &urlpath);

        // Most git services use the path for metadata in the same way, so we're going to separate
        // the metadata
        // ex. github.com/accountname/reponame
        // owner = accountname
        // name = reponame
        //
        // organizations are going to be supported on a per-host basis
        let splitpath = &urlpath.rsplit_terminator("/").collect::<Vec<&str>>();
        debug!("rsplit results for metadata: {:?}", splitpath);

        let name = splitpath[0].trim_end_matches(".git").to_string();

        let (owner, organization, fullname) = match &protocol {
            // We're not going to assume anything about metadata from a filepath
            Protocol::File => (None::<String>, None::<String>, name.clone()),
            _ => {
                // TODO: Add support for parsing out orgs from these urls
                let _hosts_w_organization_in_path =
                    vec!["ssh.dev.azure.com", "vs-ssh.visualstudio.com"];

                let mut fullname: Vec<&str> = Vec::new();

                // push organization
                // push owner
                fullname.push(splitpath[1]);
                // push name
                fullname.push(name.as_str());

                (
                    Some(splitpath[1].to_string()),
                    None::<String>,
                    fullname.join("/").to_string(),
                )
            }
        };

        Ok(GitUrl {
            href: url.to_string(),
            host: match normalized.host_str() {
                Some(h) => Some(h.to_string()),
                None => None,
            },
            name: name,
            owner: owner,
            organization: organization,
            fullname: fullname,
            protocol: Protocol::from_str(normalized.scheme()).expect("Protocol unsupported"),
            user: match normalized.username().to_string().len() {
                0 => None,
                _ => Some(normalized.username().to_string()),
            },
            token: match normalized.password() {
                Some(p) => Some(p.to_string()),
                None => None,
            },
            port: normalized.port(),
            path: urlpath,
            git_suffix: *git_suffix_check,
            ..Default::default()
        })
    }
}

/// `normalize_ssh_url` takes in an ssh url that separates the login info
/// from the path into with a `:` and replaces it with `/`.
///
/// Prepends `ssh://` to url
///
/// Supports absolute and relative paths
fn normalize_ssh_url(url: &str) -> Result<Url> {
    let u = url.split(":").collect::<Vec<&str>>();

    match u.len() {
        2 => {
            debug!("Normalizing ssh url: {:?}", u);
            normalize_url(&format!("ssh://{}/{}", u[0], u[1]))
        }
        3 => {
            debug!("Normalizing ssh url with ports: {:?}", u);
            normalize_url(&format!("ssh://{}:{}/{}", u[0], u[1], u[2]))
        }
        _default => {
            panic!("SSH normalization pattern not covered for: {:?}", u);
        }
    }
}

/// `normalize_file_path` takes in a filepath and uses `Url::from_file_path()` to parse
///
/// Prepends `file://` to url
fn normalize_file_path(filepath: &str) -> Result<Url> {
    let fp = Url::from_file_path(filepath);

    match fp {
        Ok(path) => Ok(path),
        Err(_e) => {
            Ok(normalize_url(&format!("file://{}", filepath))
                .expect("file:// normalization failed"))
        }
    }
}

/// `normalize_url` takes in url as `&str` and takes an opinionated approach to identify
/// `ssh://` or `file://` urls that require more information to be added so that
/// they can be parsed more effectively by `url::Url::parse()`
pub fn normalize_url(url: &str) -> Result<Url> {
    debug!("Processing: {:?}", &url);

    let url_parse = Url::parse(&url);

    Ok(match url_parse {
        Ok(u) => {
            match Protocol::from_str(u.scheme()) {
                Ok(_p) => u,
                Err(_e) => {
                    // Catch case when an ssh url is given w/o a user
                    debug!("Scheme parse fail. Assuming a userless ssh url");
                    normalize_ssh_url(url)?
                }
            }
        }
        Err(_e) => {
            // e will most likely be url::ParseError::RelativeUrlWithoutBase
            // If we're here, we're only looking for Protocol::Ssh or Protocol::File

            // Assuming we have found Protocol::Ssh if we can find an "@" before ":"
            // Otherwise we have Protocol::File
            let re = Regex::new(r"^\S+(@)\S+(:).*$")?;

            match re.is_match(&url) {
                true => {
                    debug!("Protocol::SSH match for normalization");
                    normalize_ssh_url(url)?
                }
                false => {
                    debug!("Protocol::File match for normalization");
                    normalize_file_path(&format!("{}", url))?
                }
            }
        }
    })
}
