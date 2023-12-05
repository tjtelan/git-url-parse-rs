use std::fmt;
use std::str::FromStr;

use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use regex::Regex;

use tracing::debug;
use url::Url;

#[derive(Debug, Error, Display)]
pub enum GitUrlError {
    /// Found null byte within input url before parsing
    NullByteFound,
    /// url parsing failed
    UrlParseFail(#[from] url::ParseError),
    /// regex failed
    RegexFail(#[from] regex::Error),
    /// SSH normalization pattern not covered for: {0}
    SshNormalizationFail(String),
    /// file:// normalization failed: {0}
    FileNormalizationFail(String),
    /// Scheme unsupported: {0}
    SchemeUnsupported(String),
    /// git url is not of expected format: {0}
    GitUrlUnsupported(String),
    /// URL host could not be represented as string
    GitUrlNoHost,
}

/// Supported uri schemes for parsing
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Display, Serialize, Deserialize)]
pub enum Scheme {
    /// Represents `file://` url scheme
    File,
    /// Represents `ftp://` url scheme
    Ftp,
    /// Represents `ftps://` url scheme
    Ftps,
    /// Represents `git://` url scheme
    Git,
    /// Represents `git+ssh://` url scheme
    #[serde(rename = "git+ssh")]
    GitSsh,
    /// Represents `http://` url scheme
    Http,
    /// Represents `https://` url scheme
    Https,
    /// Represents `ssh://` url scheme
    Ssh,
    /// Represents No url scheme
    Unspecified,
}

/// GitUrl represents an input url that is a url used by git
/// Internally during parsing the url is sanitized and uses the `url` crate to perform
/// the majority of the parsing effort, and with some extra handling to expose
/// metadata used my many git hosting services
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitUrl {
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
    /// The git url scheme
    pub scheme: Scheme,
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
    /// Indicate if url explicitly uses its scheme
    pub scheme_prefix: bool,
}

/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scheme_prefix = match self.scheme_prefix {
            true => format!("{}://", self.scheme),
            false => String::new(),
        };

        let auth_info = match self.scheme {
            Scheme::Ssh | Scheme::Git | Scheme::GitSsh => {
                if let Some(user) = &self.user {
                    format!("{}@", user)
                } else {
                    String::new()
                }
            }
            Scheme::Http | Scheme::Https => match (&self.user, &self.token) {
                (Some(user), Some(token)) => format!("{}:{}@", user, token),
                (Some(user), None) => format!("{}@", user),
                (None, Some(token)) => format!("{}@", token),
                (None, None) => String::new(),
            },
            _ => String::new(),
        };

        let host = match &self.host {
            Some(host) => host.to_string(),
            None => String::new(),
        };

        let port = match &self.port {
            Some(p) => format!(":{}", p),
            None => String::new(),
        };

        let path = match &self.scheme {
            Scheme::Ssh => {
                if self.port.is_some() {
                    format!("/{}", &self.path)
                } else {
                    format!(":{}", &self.path)
                }
            }
            _ => self.path.to_string(),
        };

        let git_url_str = format!("{}{}{}{}{}", scheme_prefix, auth_info, host, port, path);

        write!(f, "{}", git_url_str)
    }
}

impl Default for GitUrl {
    fn default() -> Self {
        GitUrl {
            host: None,
            name: "".to_string(),
            owner: None,
            organization: None,
            fullname: "".to_string(),
            scheme: Scheme::Unspecified,
            user: None,
            token: None,
            port: None,
            path: "".to_string(),
            git_suffix: false,
            scheme_prefix: false,
        }
    }
}

impl FromStr for GitUrl {
    type Err = GitUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GitUrl::parse(s)
    }
}

impl GitUrl {
    /// Returns `GitUrl` after removing `user` and `token` values
    /// Intended use-case is for non-destructive printing GitUrl excluding any embedded auth info
    pub fn trim_auth(&self) -> GitUrl {
        let mut new_giturl = self.clone();
        new_giturl.user = None;
        new_giturl.token = None;
        new_giturl
    }

    /// Returns a `Result<GitUrl>` after normalizing and parsing `url` for metadata
    pub fn parse(url: &str) -> Result<GitUrl, GitUrlError> {
        // Normalize the url so we can use Url crate to process ssh urls
        let normalized = normalize_url(url)?;

        // Some pre-processing for paths
        let scheme = if let Ok(s) = serde_plain::from_str::<Scheme>(normalized.scheme()) {
            s
        } else {
            return Err(GitUrlError::SchemeUnsupported(
                normalized.scheme().to_string(),
            ));
        };

        // Normalized ssh urls can always have their first '/' removed
        let urlpath = match &scheme {
            Scheme::Ssh => {
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
        let splitpath = &urlpath.rsplit_terminator('/').collect::<Vec<&str>>();
        debug!("rsplit results for metadata: {:?}", splitpath);

        let name = splitpath[0].trim_end_matches(".git").to_string();

        let (owner, organization, fullname) = match &scheme {
            // We're not going to assume anything about metadata from a filepath
            Scheme::File => (None::<String>, None::<String>, name.clone()),
            _ => {
                let mut fullname: Vec<&str> = Vec::new();

                // TODO: Add support for parsing out orgs from these urls
                let hosts_w_organization_in_path = ["dev.azure.com", "ssh.dev.azure.com"];
                //vec!["dev.azure.com", "ssh.dev.azure.com", "visualstudio.com"];

                let host_str = if let Some(host) = normalized.host_str() {
                    host
                } else {
                    return Err(GitUrlError::GitUrlNoHost);
                };

                match hosts_w_organization_in_path.contains(&host_str) {
                    true => {
                        debug!("Found a git provider with an org");

                        // The path differs between git:// and https:// schemes

                        match &scheme {
                            // Example: "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName",
                            Scheme::Ssh => {
                                // Organization
                                fullname.push(splitpath[2]);
                                // Project/Owner name
                                fullname.push(splitpath[1]);
                                // Repo name
                                fullname.push(splitpath[0]);

                                (
                                    Some(splitpath[1].to_string()),
                                    Some(splitpath[2].to_string()),
                                    fullname.join("/"),
                                )
                            }
                            // Example: "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName",
                            Scheme::Https => {
                                // Organization
                                fullname.push(splitpath[3]);
                                // Project/Owner name
                                fullname.push(splitpath[2]);
                                // Repo name
                                fullname.push(splitpath[0]);

                                (
                                    Some(splitpath[2].to_string()),
                                    Some(splitpath[3].to_string()),
                                    fullname.join("/"),
                                )
                            }
                            _e => return Err(GitUrlError::SchemeUnsupported(_e.to_string())),
                        }
                    }
                    false => {
                        if !url.starts_with("ssh") && splitpath.len() < 2 {
                            return Err(GitUrlError::GitUrlUnsupported(url.to_string()));
                        }

                        let position = match splitpath.len() {
                            0 => return Err(GitUrlError::GitUrlUnsupported(url.to_string())),
                            1 => 0,
                            _ => 1,
                        };

                        // push owner
                        fullname.push(splitpath[position]);
                        // push name
                        fullname.push(name.as_str());

                        (
                            Some(splitpath[position].to_string()),
                            None::<String>,
                            fullname.join("/"),
                        )
                    }
                }
            }
        };

        let final_host = match scheme {
            Scheme::File => None,
            _ => normalized.host_str().map(|h| h.to_string()),
        };

        let final_path = match scheme {
            Scheme::File => {
                if let Some(host) = normalized.host_str() {
                    format!("{}{}", host, urlpath)
                } else {
                    urlpath
                }
            }
            _ => urlpath,
        };

        Ok(GitUrl {
            host: final_host,
            name,
            owner,
            organization,
            fullname,
            scheme,
            user: match normalized.username().to_string().len() {
                0 => None,
                _ => Some(normalized.username().to_string()),
            },
            token: normalized.password().map(|p| p.to_string()),
            port: normalized.port(),
            path: final_path,
            git_suffix: *git_suffix_check,
            scheme_prefix: url.contains("://") || url.starts_with("git:"),
        })
    }
}

/// `normalize_ssh_url` takes in an ssh url that separates the login info
/// from the path into with a `:` and replaces it with `/`.
///
/// Prepends `ssh://` to url
///
/// Supports absolute and relative paths
fn normalize_ssh_url(url: &str) -> Result<Url, GitUrlError> {
    let u = url.split(':').collect::<Vec<&str>>();

    match u.len() {
        2 => {
            debug!("Normalizing ssh url: {:?}", u);
            normalize_url(&format!("ssh://{}/{}", u[0], u[1]))
        }
        3 => {
            debug!("Normalizing ssh url with ports: {:?}", u);
            normalize_url(&format!("ssh://{}:{}/{}", u[0], u[1], u[2]))
        }
        _default => Err(GitUrlError::SshNormalizationFail(url.to_string())),
    }
}

/// `normalize_file_path` takes in a filepath and uses `Url::from_file_path()` to parse
///
/// Prepends `file://` to url
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi"))]
fn normalize_file_path(filepath: &str) -> Result<Url, GitUrlError> {
    let fp = Url::from_file_path(filepath);

    let url = match fp {
        Ok(path) => Ok(path),
        Err(_e) => normalize_url(&format!("file://{}", filepath)),
    };

    match url {
        Ok(u) => Ok(u),
        Err(_) => Err(GitUrlError::FileNormalizationFail(filepath.to_string())),
    }
}

#[cfg(target_arch = "wasm32")]
fn normalize_file_path(_filepath: &str) -> Result<Url, GitUrlError> {
    unreachable!()
}

/// `normalize_url` takes in url as `&str` and takes an opinionated approach to identify
/// `ssh://` or `file://` urls that require more information to be added so that
/// they can be parsed more effectively by `url::Url::parse()`
pub fn normalize_url(url: &str) -> Result<Url, GitUrlError> {
    debug!("Processing: {:?}", &url);

    // Error if there are null bytes within the url
    // https://github.com/tjtelan/git-url-parse-rs/issues/16
    if url.contains('\0') {
        return Err(GitUrlError::NullByteFound);
    }

    // We're going to remove any trailing slash before running through Url::parse
    let trim_url = url.trim_end_matches('/');

    let regex_check = match Regex::new(r"^git:[^/]") {
        Ok(re) => re,
        Err(e) => return Err(GitUrlError::RegexFail(e)),
    };

    // normalize short git url notation: git:host/path
    let url_to_parse = if regex_check.is_match(trim_url) {
        trim_url.replace("git:", "git://")
    } else {
        trim_url.to_string()
    };

    let url_parse = Url::parse(&url_to_parse);

    Ok(match url_parse {
        Ok(u) => {
            match serde_plain::from_str::<Scheme>(u.scheme()) {
                Ok(_p) => u,
                Err(_e) => {
                    // Catch case when an ssh url is given w/o a user
                    debug!("Scheme parse fail. Assuming a userless ssh url");
                    normalize_ssh_url(trim_url)?
                }
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // If we're here, we're only looking for Scheme::Ssh or Scheme::File

            // Assuming we have found Scheme::Ssh if we can find an "@" before ":"
            // Otherwise we have Scheme::File
            let re = Regex::new(r"^\S+(@)\S+(:).*$")?;

            match re.is_match(trim_url) {
                true => {
                    debug!("Scheme::SSH match for normalization");
                    normalize_ssh_url(trim_url)?
                }
                false => {
                    debug!("Scheme::File match for normalization");
                    normalize_file_path(trim_url)?
                }
            }
        }
        Err(err) => {
            return Err(GitUrlError::UrlParseFail(err));
        }
    })
}
