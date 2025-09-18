//! # GitUrl internal types
//!
//! Internal types and parsing logic for Git urls
//!

mod error;
mod spec;
use spec::*;
pub mod provider;

pub use error::GitUrlParseError;

use core::str;
use std::fmt;
use url::Url;

use getset::{CopyGetters, Getters, Setters};
#[cfg(feature = "log")]
use log::debug;
use nom::Finish;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Assigned as a label during parsing for different Git URL types.
/// Some printing or `GitProvider` parsing behavior are influenced by this type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) enum GitUrlParseHint {
    /// The default status
    #[default]
    Unknown,
    /// When `ssh` is in the scheme, or a `:` is used as initial path separator
    Sshlike,
    /// When `file` is in scheme, or filesystem-like relative paths
    Filelike,
    /// Default network scheme if not `ssh`. If `:` is used as initial path separator in the userinfo
    Httplike,
}

/// Represents a parsed Git repository url
///
/// GitUrl is an input url used by git.
/// Parsing of the url inspired by rfc3986, but does not strictly cover the spec
/// Optional, but by default, uses the `url` crate to perform a final validation of the parsing effort
#[derive(Clone, CopyGetters, Getters, Debug, Default, Setters, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(set = "pub(crate)")]
pub struct GitUrl {
    /// scheme name (i.e. `scheme://`)
    scheme: Option<String>,
    /// user name userinfo
    user: Option<String>,
    /// password userinfo provided with `user` (i.e. `user`:`password`@...)
    password: Option<String>,
    /// The hostname or IP of the repo host
    host: Option<String>,
    /// The port number of the repo host, if specified
    #[getset(get_copy = "pub")]
    port: Option<u16>,
    /// File or network path to repo
    path: String,
    /// If we should print `scheme://` from input or derived during parsing
    #[getset(get_copy = "pub")]
    print_scheme: bool,
    /// Pattern style of url derived during parsing
    #[getset(get_copy = "pub")]
    hint: GitUrlParseHint,
}

impl GitUrl {
    /// scheme name (i.e. `scheme://`)
    pub fn scheme(&self) -> Option<&str> {
        if let Some(s) = &self.scheme {
            Some(&s[..])
        } else {
            None
        }
    }

    /// user name userinfo
    pub fn user(&self) -> Option<&str> {
        if let Some(u) = &self.user {
            Some(&u[..])
        } else {
            None
        }
    }

    /// password userinfo provided with `user` (i.e. `user`:`password`@...)
    pub fn password(&self) -> Option<&str> {
        if let Some(p) = &self.password {
            Some(&p[..])
        } else {
            None
        }
    }

    /// The hostname or IP of the repo host
    pub fn host(&self) -> Option<&str> {
        if let Some(h) = &self.host {
            Some(&h[..])
        } else {
            None
        }
    }

    /// File or network path to repo
    pub fn path(&self) -> &str {
        &self.path[..]
    }

    /// Wrapper function for the default output mode via [`Display`](std::fmt::Display) trait
    fn display(&self) -> String {
        self.build_string(false)
    }

    /// Wrapper function for printing a url for the [`url`](https://docs.rs/url/latest/url/) crate
    #[cfg(feature = "url")]
    fn url_compat_display(&self) -> String {
        self.build_string(true)
    }

    /// This method rebuilds the printable GitUrl from its components.
    /// `url_compat` results in output that can be parsed by the [`url`](https://docs.rs/url/latest/url/) crate
    fn build_string(&self, url_compat: bool) -> String {
        let scheme = if self.print_scheme() || url_compat {
            if let Some(scheme) = self.scheme() {
                format!("{scheme}://")
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let auth_info = match (self.user(), self.password()) {
            (Some(user), Some(password)) => format!("{user}:{password}@"),
            (Some(user), None) => format!("{user}@",),
            (None, Some(password)) => format!("{password}@"),
            (None, None) => String::new(),
        };

        let host = match &self.host() {
            Some(host) => host.to_string(),
            None => String::new(),
        };

        let (port, path) = match (self.hint(), self.port(), self.path()) {
            (GitUrlParseHint::Httplike, Some(port), path) => {
                (format!(":{port}"), format!("/{path}"))
            }
            (GitUrlParseHint::Httplike, None, path) => (String::new(), path.to_string()),
            (GitUrlParseHint::Sshlike, Some(port), path) => {
                (format!(":{port}"), format!("/{path}"))
            }
            (GitUrlParseHint::Sshlike, None, path) => {
                if url_compat {
                    (String::new(), format!("/{path}"))
                } else {
                    (String::new(), format!(":{path}"))
                }
            }
            (GitUrlParseHint::Filelike, None, path) => (String::new(), path.to_string()),
            _ => (String::new(), String::new()),
        };

        let git_url_str = format!("{scheme}{auth_info}{host}{port}{path}");
        git_url_str
    }

    /// Returns `GitUrl` after removing all user info values
    pub fn trim_auth(&self) -> GitUrl {
        let mut new_giturl = self.clone();
        new_giturl.set_user(None);
        new_giturl.set_password(None);
        #[cfg(feature = "log")]
        debug!("{new_giturl:?}");
        new_giturl
    }

    /// Returns a `Result<GitUrl>` after parsing `input` for metadata
    ///
    /// ```
    /// # use git_url_parse::GitUrl;
    /// # use git_url_parse::types::provider::GenericProvider;
    /// # fn main() -> Result<(), git_url_parse::GitUrlParseError> {
    /// let http_url = GitUrl::parse("https://github.com/tjtelan/git-url-parse-rs.git")?;
    /// let ssh_url = GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git")?;
    /// # Ok(())
    /// #  }
    /// ```
    pub fn parse(input: &str) -> Result<Self, GitUrlParseError> {
        let git_url = Self::parse_to_git_url(input)?;

        git_url.is_valid()?;

        Ok(git_url)
    }

    /// Internal parse to `GitUrl` without validation steps
    fn parse_to_git_url(input: &str) -> Result<Self, GitUrlParseError> {
        let mut git_url_result = GitUrl::default();
        // Error if there are null bytes within the url
        // https://github.com/tjtelan/git-url-parse-rs/issues/16
        if input.contains('\0') {
            return Err(GitUrlParseError::FoundNullBytes);
        }

        let (_input, url_spec_parser) = UrlSpecParser::parse(input).finish().unwrap_or_default();

        let scheme = url_spec_parser.scheme();
        let user = url_spec_parser.hier_part().authority().userinfo().user();
        let password = url_spec_parser.hier_part().authority().userinfo().token();
        let host = url_spec_parser.hier_part().authority().host();
        let port = url_spec_parser.hier_part().authority().port();
        let path = url_spec_parser.hier_part().path();

        git_url_result.set_scheme(scheme.clone());
        git_url_result.set_user(user.clone());
        git_url_result.set_password(password.clone());
        git_url_result.set_host(host.clone());
        git_url_result.set_port(*port);
        git_url_result.set_path(path.clone());

        // We will respect whether scheme was initially set
        let print_scheme = scheme.is_some();

        // Take a moment to identify the type of url we have
        // We use the GitUrlParseHint to validate or adjust formatting path, if necessary
        let hint = if let Some(scheme) = scheme.as_ref() {
            if scheme.contains("ssh") {
                GitUrlParseHint::Sshlike
            } else {
                match scheme.to_lowercase().as_str() {
                    "file" => GitUrlParseHint::Filelike,
                    _ => GitUrlParseHint::Httplike,
                }
            }
        } else if user.is_none()
            && password.is_none()
            && host.is_none()
            && port.is_none()
            && !path.is_empty()
        {
            // if we only have a path => file
            GitUrlParseHint::Filelike
        } else if user.is_some() && password.is_some() {
            // If we have a user and password => http
            GitUrlParseHint::Httplike
        } else if path.starts_with(':') {
            // If path starts with a colon => ssh
            GitUrlParseHint::Sshlike
        } else {
            GitUrlParseHint::Unknown
        };

        // If we found an ssh url, we should adjust the path.
        // Skip the first character
        if hint == GitUrlParseHint::Sshlike {
            git_url_result.set_scheme(Some("ssh".to_string()));
            git_url_result.set_path(path[1..].to_string());
        }

        if hint == GitUrlParseHint::Filelike {
            git_url_result.set_scheme(Some("file".to_string()));
        }

        git_url_result.set_print_scheme(print_scheme);
        git_url_result.set_hint(hint);

        git_url_result.is_valid()?;

        Ok(git_url_result)
    }

    /// Normalize input into form that can be used by [`Url::parse`](https://docs.rs/url/latest/url/struct.Url.html#method.parse)
    #[cfg(feature = "url")]
    pub fn parse_to_url(input: &str) -> Result<Url, GitUrlParseError> {
        let git_url = Self::parse_to_git_url(input)?;

        Ok(Url::try_from(git_url)?)
    }

    /// ```
    /// use git_url_parse::GitUrl;
    /// use git_url_parse::types::provider::GenericProvider;
    ///
    /// # fn main() -> Result<(), git_url_parse::GitUrlParseError> {
    /// let ssh_url = GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git")?;
    /// let provider : GenericProvider = ssh_url.provider_info()?;
    /// # assert_eq!(provider.owner(), "tjtelan");
    /// # assert_eq!(provider.repo(), "git-url-parse-rs");
    ///
    /// # Ok(())
    /// # }
    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
    where
        T: provider::GitProvider<GitUrl, GitUrlParseError>,
    {
        T::from_git_url(self)
    }

    /// This is called as the last step before returning a `GitUrl` to the user
    fn is_valid(&self) -> Result<(), GitUrlParseError> {
        // Last chance validation

        #[cfg(feature = "log")]
        debug!("Validating parsing results {self:#?}");

        if self.path().is_empty() {
            return Err(GitUrlParseError::InvalidPathEmpty);
        }

        // There's an edge case we don't properly cover: ssh urls using ports + absolute paths
        // https://mslinn.com/git/040-git-urls.html - describes this pattern, if we decide to parse for it

        // only ssh paths start with ':'
        if self.hint() != GitUrlParseHint::Sshlike && self.path.starts_with(':') {
            #[cfg(feature = "log")]
            {
                debug!("{:?}", self.hint());
                debug!("{:?}", self.path());
                debug!("Only sshlike url path starts with ':'");
                debug!("path starts with ':'? {}", self.path.starts_with(':'));
            }

            return Err(GitUrlParseError::InvalidPortNumber);
        }

        // if we are not httplike, we shouldn't have passwords
        if self.hint() != GitUrlParseHint::Httplike && self.password().is_some() {
            #[cfg(feature = "log")]
            {
                debug!("{:?}", self.hint());
                debug!(
                    "password support only for httplike url: {:?}",
                    self.password()
                );
            }
            return Err(GitUrlParseError::InvalidPasswordUnsupported);
        }

        // if we are filelike, we should only have paths
        if self.hint() == GitUrlParseHint::Filelike
            && (self.user().is_some()
                || self.password().is_some()
                || self.host().is_some()
                || self.port().is_some()
                || self.path().is_empty())
        {
            #[cfg(feature = "log")]
            {
                debug!(
                    "Only scheme and path expected to have values set for filelike urls {:?}",
                    self
                );
            }
            return Err(GitUrlParseError::InvalidFilePattern);
        }

        #[cfg(feature = "url")]
        {
            // Since we don't fully implement any spec, we'll rely on the url crate
            let _u: Url = self.try_into()?;
        }

        Ok(())
    }
}

/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let git_url_str = self.display();

        write!(f, "{git_url_str}",)
    }
}

#[cfg(feature = "url")]
impl TryFrom<&GitUrl> for Url {
    type Error = url::ParseError;
    fn try_from(value: &GitUrl) -> Result<Self, Self::Error> {
        // Since we don't fully implement any spec, we'll rely on the url crate
        Url::parse(&value.url_compat_display())
    }
}

#[cfg(feature = "url")]
impl TryFrom<GitUrl> for Url {
    type Error = url::ParseError;
    fn try_from(value: GitUrl) -> Result<Self, Self::Error> {
        // Since we don't fully implement any spec, we'll rely on the url crate
        Url::parse(&value.url_compat_display())
    }
}

#[cfg(feature = "url")]
impl TryFrom<&Url> for GitUrl {
    type Error = GitUrlParseError;
    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        GitUrl::parse(value.as_str())
    }
}

#[cfg(feature = "url")]
impl TryFrom<Url> for GitUrl {
    type Error = GitUrlParseError;
    fn try_from(value: Url) -> Result<Self, Self::Error> {
        GitUrl::parse(value.as_str())
    }
}
