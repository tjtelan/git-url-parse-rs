mod error;
mod provider;

pub use error::GitUrlParseError;
pub use provider::{GenericProvider, GitProvider, AzureDevOpsProvider, GitLabProvider};

use derive_builder::Builder;
use getset::{Getters, Setters};
use strum::{Display, EnumString, VariantNames};

use core::str;
use std::fmt;
use std::str::FromStr;
//use url::Url;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::one_of;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser, combinator::opt, combinator::rest};

/// Supported uri schemes for parsing
#[derive(Debug, PartialEq, Eq, EnumString, VariantNames, Clone, Display)]
#[strum(serialize_all = "kebab_case")]
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
    #[strum(serialize = "git+ssh")]
    GitSsh,
    /// Represents `http://` url scheme
    Http,
    /// Represents `https://` url scheme
    Https,
    /// Represents `ssh://` url scheme
    Ssh,
    ///// Represents No url scheme
    //Unspecified,
    ///
    Other(String), // todo: need test for this
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum GitUrlParseHint {
    #[default]
    Unknown,
    Sshlike,
    Filelike,
    Httplike,
    //Custom // needed?
}

/// GitUrl represents an input url that is a url used by git
/// Internally during parsing the url is sanitized and uses the `url` crate to perform
/// the majority of the parsing effort, and with some extra handling to expose
/// metadata used my many git hosting services
#[derive(Debug, PartialEq, Eq, Clone, Builder, Default, Getters, Setters)]
#[builder(build_fn(validate = "Self::prebuild_check"), field(public))]
#[get = "pub"]
pub struct GitUrl<P = GenericProvider>
where
    P: GitProvider<GitUrl, GitUrlParseError>,
{
    /// The fully qualified domain name (FQDN) or IP of the repo
    #[builder(setter(into, strip_option), default)]
    host: Option<String>,
    ///// The name of the repo
    //pub name: String,
    ///// The owner/account/project name
    //pub owner: Option<String>,
    ///// The organization name. Supported by Azure DevOps
    //pub organization: Option<String>,
    ///// The full name of the repo, formatted as "owner/name"
    //pub fullname: String,
    ///// The git url scheme
    #[builder(setter(into, strip_option), default)]
    scheme: Option<Scheme>,
    /// The authentication user
    #[builder(setter(into, strip_option), default)]
    #[getset(set = "pub(crate)")]
    user: Option<String>,
    /// The oauth token (could appear in the https urls)
    #[builder(setter(into, strip_option), default)]
    #[getset(set = "pub(crate)")]
    token: Option<String>,
    /// The non-conventional port where git service is hosted
    #[builder(setter(into, strip_option), default)]
    port: Option<u16>,
    /// The path to repo w/ respect to user + hostname
    #[builder(setter(into))]
    path: String,
    ///// Indicate if url uses the .git suffix
    //pub git_suffix: bool,
    ///// Indicate if url explicitly uses its scheme
    //pub scheme_prefix: bool,
    #[builder(default)]
    print_scheme: bool,

    #[builder(setter(into, strip_option), default)]
    provider: Option<P>,
}

impl<P: GitProvider<GitUrl, GitUrlParseError>> GitUrlBuilder<P> {
    pub fn trim_auth(&mut self) {
        self.user = None;
        self.token = None;
    }

    fn prebuild_check(&self) -> Result<(), String> {
        #[cfg(feature = "tracing")]
        debug!("Processing: {:?}", &url);

        // Error if there are null bytes within the url

        // https://github.com/tjtelan/git-url-parse-rs/issues/16
        if let Some(Some(host)) = &self.host {
            if host.contains('\0') {
                return Err(GitUrlParseError::FoundNullBytes.to_string());
            }

            if host.is_empty() {
                return Err(
                    GitUrlParseError::UnexpectedEmptyValue(String::from("host")).to_string()
                );
            }
        }

        if let Some(Some(user)) = &self.user {
            if user.contains('\0') {
                return Err(GitUrlParseError::FoundNullBytes.to_string());
            }

            if user.is_empty() {
                return Err(
                    GitUrlParseError::UnexpectedEmptyValue(String::from("user")).to_string()
                );
            }
        }

        if let Some(Some(token)) = &self.token {
            if token.contains('\0') {
                return Err(GitUrlParseError::FoundNullBytes.to_string());
            }

            if token.is_empty() {
                return Err(
                    GitUrlParseError::UnexpectedEmptyValue(String::from("token")).to_string(),
                );
            }
        }

        if let Some(path) = &self.path {
            if path.contains('\0') {
                return Err(GitUrlParseError::FoundNullBytes.to_string());
            }
            if path.is_empty() {
                return Err(
                    GitUrlParseError::UnexpectedEmptyValue(String::from("path")).to_string()
                );
            }
        }

        Ok(())
    }

    fn parse(url: &str) -> Result<Self, GitUrlParseError> {
        println!("start: {url}");
        let mut giturl = GitUrlBuilder::default();
        let mut working_url = url;
        let mut hint = GitUrlParseHint::default();

        giturl.parse_scheme(&mut working_url, &mut hint);
        giturl.parse_auth_info(&mut working_url, &mut hint);
        let save_state = working_url;

        giturl.parse_host_port(&mut working_url, &mut hint);

        match hint {
            GitUrlParseHint::Httplike => {}
            GitUrlParseHint::Sshlike => {
                //working_url = giturl.parse_ssh_path(&working_url);
                giturl.parse_ssh_path(&mut working_url, &mut hint);
            }
            GitUrlParseHint::Filelike | GitUrlParseHint::Unknown => {
                working_url = save_state;
                giturl.host = None;
                giturl.scheme(Scheme::File);
            }
        }

        giturl.parse_path(&mut working_url, &mut hint);

        println!("");
        Ok(giturl)
    }

    fn parse_scheme(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
        let mut builder = self.clone();

        if let Ok((leftover, Some(s))) = GitUrlBuilder::<P>::_parse_scheme(working_url) {
            println!("leftover: {leftover}, scheme: {s:?}");

            let scheme = Scheme::from_str(s).expect("Unknown scheme");

            *hint = match &scheme {
                Scheme::Ssh => GitUrlParseHint::Sshlike,
                Scheme::File => GitUrlParseHint::Filelike,
                _ => GitUrlParseHint::Httplike,
            };

            builder.scheme(scheme);
            builder.print_scheme(true);

            *self = builder;
            *working_url = leftover;
        }
    }

    fn parse_auth_info(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
        let mut builder = self.clone();
        if let Ok((leftover, Some(username))) = GitUrlBuilder::<P>::_parse_username(working_url) {
            println!("leftover: {leftover}, username: {username:?}");
            builder.user(username);

            if *hint == GitUrlParseHint::Unknown {
                *hint = GitUrlParseHint::Sshlike;
            }

            if let Ok((token, Some(real_username))) = GitUrlBuilder::<P>::_parse_token(username) {
                println!("token: {token}, real_username: {real_username:?}");
                builder.user(real_username);
                builder.token(token);

                if *hint == GitUrlParseHint::Unknown || *hint == GitUrlParseHint::Sshlike {
                    *hint = GitUrlParseHint::Httplike;
                }
            }

            *working_url = leftover;
            *self = builder;
        }
    }

    fn parse_host_port(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
        let mut builder = self.clone();
        let mut save = working_url.clone();

        if let Ok((leftover, Some(hostname))) = GitUrlBuilder::<P>::_parse_hostname(save) {
            println!("leftover {leftover}, hostname: {hostname}");
            builder.host(hostname);
            save = leftover;
        }

        if let Ok((leftover, Some(port))) = GitUrlBuilder::<P>::_parse_port(save) {
            if !port.is_empty() {
                println!("leftover {leftover}, port: {port}");
                builder.port(u16::from_str(port).expect("Not a valid port"));
                save = leftover;

                // If we're currently uncertain, but we've found a port
                // our guess is this more likely is an http url than an ssh url
                // Add the `ssh://` scheme to the url if this is incorrect
                if *hint == GitUrlParseHint::Unknown {
                    *hint = GitUrlParseHint::Httplike;
                }
            }
        }

        // https://mslinn.com/git/040-git-urls.html - we only support relative paths when we have ports
        if builder.port.is_none() && save.starts_with(":") {
            *hint = GitUrlParseHint::Sshlike;
        }

        *self = builder;
        *working_url = save;
    }

    fn parse_ssh_path(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
        let mut builder = self.clone();

        if let Ok((_leftover, Some(path))) = GitUrlBuilder::<P>::_parse_ssh_path(working_url) {
            builder.scheme(Scheme::Ssh);

            *self = builder;
            *working_url = path;
        }
    }

    fn parse_path(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
        let mut builder = self.clone();
        if let Ok((leftover, path)) = GitUrlBuilder::<P>::_parse_path(working_url) {
            println!("leftover {leftover}, path: {path}");

            builder.path(path);

            *self = builder;
            *working_url = leftover;
        }
    }

    ////

    fn _parse_scheme(input: &str) -> IResult<&str, Option<&str>> {
        opt(terminated(
            alt((
                tag(Scheme::File.to_string().as_bytes()),
                tag(Scheme::Ftps.to_string().as_bytes()),
                tag(Scheme::Ftp.to_string().as_bytes()),
                tag(Scheme::GitSsh.to_string().as_bytes()),
                tag(Scheme::Git.to_string().as_bytes()),
                tag(Scheme::Https.to_string().as_bytes()),
                tag(Scheme::Http.to_string().as_bytes()),
                tag(Scheme::Ssh.to_string().as_bytes()),
                // todo: Other(), needs a test
            )),
            tag("://"),
        ))
        .parse(input)
    }

    fn _parse_username(input: &str) -> IResult<&str, Option<&str>> {
        opt(terminated(take_until("@"), tag("@"))).parse(input)
    }

    fn _parse_token(input: &str) -> IResult<&str, Option<&str>> {
        opt(terminated(take_until(":"), tag(":"))).parse(input)
    }

    fn _parse_hostname(input: &str) -> IResult<&str, Option<&str>> {
        opt(take_till(|c| c == '/' || c == ':')).parse(input)
    }

    fn _parse_port(input: &str) -> IResult<&str, Option<&str>> {
        opt(preceded(tag(":"), take_while(|c: char| c.is_digit(10)))).parse(input)
    }

    // This is making an assumption that the path is relative, not absolute
    // This is bc we do not support absolute paths when we also have a port
    fn _parse_ssh_path(input: &str) -> IResult<&str, Option<&str>> {
        opt(preceded(one_of("/:"), rest)).parse(input)
    }

    fn _parse_path(input: &str) -> IResult<&str, &str> {
        rest(input)
    }
}

/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scheme = if let Some(scheme) = &self.scheme()
            && self.print_scheme().clone()
        {
            format!("{}://", scheme)
        } else {
            String::new()
        };

        let auth_info = match self.scheme() {
            Some(Scheme::Ssh) | Some(Scheme::Git) | Some(Scheme::GitSsh) => {
                if let Some(user) = &self.user() {
                    format!("{user}@")
                } else {
                    String::new()
                }
            }
            Some(Scheme::Http) | Some(Scheme::Https) => match (&self.user(), &self.token()) {
                (Some(user), Some(token)) => format!("{user}:{token}@"),
                (Some(user), None) => format!("{user}@",),
                (None, Some(token)) => format!("{token}@"),
                (None, None) => String::new(),
            },
            _ => String::new(),
        };

        let host = match &self.host() {
            Some(host) => host.to_string(),
            None => String::new(),
        };

        let port = match &self.port() {
            Some(p) => format!(":{}", p),
            None => String::new(),
        };

        let path = if self.scheme().clone() == Some(Scheme::Ssh) {
            if self.port().is_some() {
                if !self.path().starts_with('/') {
                    format!("/{}", &self.path())
                } else {
                    self.path().to_string()
                }
            } else {
                format!(":{}", &self.path())
            }
        } else {
            self.path().to_string()
        };

        let git_url_str = format!("{scheme}{auth_info}{host}{port}{path}");

        write!(f, "{}", git_url_str)
    }
}

impl FromStr for GitUrl {
    //type Err = GitUrlParseError;
    type Err = GitUrlBuilderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GitUrl::parse(s)
    }
}

impl GitUrl {
    /// Returns `GitUrl` after removing `user` and `token` values
    /// Intended use-case is for non-destructive printing GitUrl excluding any embedded auth info
    pub fn trim_auth(&self) -> GitUrl {
        let mut new_giturl = self.clone();
        new_giturl.set_user(None);
        new_giturl.set_token(None);
        new_giturl
    }

    /// Returns a `Result<GitUrl>` after normalizing and parsing `url` for metadata
    pub fn parse(url: &str) -> Result<GitUrl, GitUrlBuilderError> {
        let giturl = GitUrlBuilder::parse(url).unwrap();
        giturl.build()
    }

    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
    where
        T: GitProvider<GitUrl, GitUrlParseError>,
    {
        T::from_git_url(&self)
        //Err(GitUrlParseError::UnexpectedFormat)
    }
}