mod error;
mod spec;
use spec::*;
pub mod provider;

pub use error::GitUrlParseError;

use core::str;
use std::fmt;

use getset::{CloneGetters, CopyGetters, Setters};
use nom::Finish;

#[cfg(feature = "tracing")]
use tracing::debug;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum GitUrlParseHint {
    #[default]
    Unknown,
    Sshlike,
    Filelike,
    Httplike,
}

#[derive(Clone, CopyGetters, CloneGetters, Debug, Default, Setters, PartialEq, Eq)]
pub struct GitUrl<'url> {
    #[getset(get_copy = "pub", set = "pub(crate)")]
    scheme: Option<&'url str>,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    user: Option<&'url str>,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    token: Option<&'url str>,
    #[getset(get_copy = "pub")]
    host: Option<&'url str>,
    #[getset(get_copy = "pub")]
    port: Option<u16>,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    path: &'url str,
    /// Include scheme:// when printing url
    #[getset(get_copy = "pub", set = "pub(crate)")]
    print_scheme: bool,
    #[getset(get_copy = "pub(crate)")]
    hint: GitUrlParseHint,
}

/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let git_url_str = self.display();

        write!(f, "{git_url_str}",)
    }
}

impl<'url> GitUrl<'url> {
    fn display(&self) -> String {
        self.build_string(false)
    }

    fn url_compat_display(&self) -> String {
        self.build_string(true)
    }

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

        let auth_info = match (self.user(), self.token()) {
            (Some(user), Some(token)) => format!("{user}:{token}@"),
            (Some(user), None) => format!("{user}@",),
            (None, Some(token)) => format!("{token}@"),
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
}

impl<'url> GitUrl<'url> {
    /// Returns `GitUrl` after removing `user` and `token` values
    /// Intended use-case is for non-destructive printing GitUrl excluding any embedded auth info
    pub fn trim_auth(&self) -> GitUrl {
        let mut new_giturl = self.clone();
        new_giturl.set_user(None);
        new_giturl.set_token(None);
        #[cfg(feature = "tracing")]
        debug!(?new_giturl);
        new_giturl
    }

    pub fn parse(input: &'url str) -> Result<Self, GitUrlParseError> {
        // Error if there are null bytes within the url
        // https://github.com/tjtelan/git-url-parse-rs/issues/16
        if input.contains('\0') {
            return Err(GitUrlParseError::FoundNullBytes);
        }

        let (_input, url_spec_parser) = UrlSpecParser::parse(input).finish().unwrap_or_default();

        let mut scheme = url_spec_parser.scheme();
        let user = url_spec_parser.heir_part().authority().userinfo().user();
        let token = url_spec_parser.heir_part().authority().userinfo().token();
        let host = url_spec_parser.heir_part().authority().host();
        let port = url_spec_parser.heir_part().authority().port();
        let mut path = url_spec_parser.heir_part().path();

        // We will respect whether scheme was initially set
        let print_scheme = scheme.is_some();

        // Take a moment to identify the type of url we have
        // We use the GitUrlParseHint to validate or adjust formatting path, if necessary
        let hint = if let Some(scheme) = scheme {
            if scheme.contains("ssh") {
                GitUrlParseHint::Sshlike
            } else {
                match scheme.to_lowercase().as_str() {
                    "file" => GitUrlParseHint::Filelike,
                    _ => GitUrlParseHint::Httplike,
                }
            }
        } else if user.is_none()
            && token.is_none()
            && host.is_none()
            && port.is_none()
            && !path.is_empty()
        {
            // if we only have a path => file
            GitUrlParseHint::Filelike
        } else if user.is_some() && token.is_some() {
            // If we have a user and token => http
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
            if let Some(scheme) = scheme.as_mut() {
                *scheme = "ssh";
            } else {
                scheme = Some("ssh")
            }
            path = &path[1..];
        }

        if hint == GitUrlParseHint::Filelike {
            if let Some(scheme) = scheme.as_mut() {
                *scheme = "file";
            } else {
                scheme = Some("file")
            }
        }

        let git_url = GitUrl {
            scheme,
            user,
            token,
            host,
            port,
            path,
            print_scheme,
            hint,
        };

        let _check = git_url.is_valid()?;

        Ok(git_url)
    }

    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
    where
        T: provider::GitProvider<GitUrl<'url>, GitUrlParseError>,
    {
        T::from_git_url(self)
    }

    fn is_valid(&self) -> Result<(), GitUrlParseError> {
        // Last chance validation

        //println!("{self:#?}");

        if self.path().is_empty() {
            return Err(GitUrlParseError::InvalidPathEmpty);
        }

        // There's an edge case we don't cover: ssh urls using ports + absolute paths
        // https://mslinn.com/git/040-git-urls.html - describes this pattern, if we decide to parse for it

        // only ssh paths start with ':'
        if self.hint() != GitUrlParseHint::Sshlike && self.path.starts_with(':') {
            #[cfg(feature = "tracing")]
            {
                debug!("{}", self.hint());
                debug!(self.path);
                debug!("Only sshlike url path starts with ':'");
                debug!("path starts with ':'?", self.path.starts_with(':'));
            }

            return Err(GitUrlParseError::InvalidPortNumber);
        }

        // if we are not httplike, we shouldn't have tokens
        if self.hint() != GitUrlParseHint::Httplike && self.token().is_some() {
            #[cfg(feature = "tracing")]
            {
                debug!("{}", self.hint());
                debug!("Token support only for httplike url", self.token());
            }
            return Err(GitUrlParseError::InvalidTokenUnsupported);
        }

        // if we are filelike, we should only have paths
        if self.hint() == GitUrlParseHint::Filelike
            && (self.user().is_some()
                || self.token().is_some()
                || self.host().is_some()
                || self.port().is_some()
                || self.path().is_empty())
        {
            #[cfg(feature = "tracing")]
            {
                debug!(
                    "Only scheme and path expected to have values set for filelike urls",
                    ?self
                );
            }
            return Err(GitUrlParseError::InvalidFilePattern);
        }

        // Since we don't fully implement any spec, we'll rely on the url crate
        println!("{:#?}", self.url_compat_display());
        let _u = url::Url::parse(&self.url_compat_display())?;

        Ok(())
    }
}
