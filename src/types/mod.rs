mod error;
mod provider;

pub use error::GitUrlParseError;
pub use provider::{AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider};

use core::str;
use std::fmt;

use getset::{CloneGetters, CopyGetters, Setters};
use nom::Finish;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map_opt, peek, recognize, verify};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt};

#[cfg(feature = "tracing")]
use tracing::debug;
use typed_path::{Utf8TypedPath, Utf8TypedPathBuf};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum GitUrlParseHint {
    #[default]
    Unknown,
    Sshlike,
    Filelike,
    Httplike,
}

#[derive(Clone, CopyGetters, CloneGetters, Setters, Default, PartialEq, Eq)]
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
    #[getset(get_copy = "pub")]
    path: &'url str,

    //#[getset(skip)]
    //url: String,
    /// Include scheme:// when printing url
    #[getset(get_copy = "pub")]
    print_scheme: bool,

    #[getset(get_copy = "pub(crate)")]
    hint: GitUrlParseHint,
    ///// Hosted git provider info derived from GitUrl
    //#[getset(skip)]
    //provider: Option<P>,
}

/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scheme = if self.print_scheme() {
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
            (GitUrlParseHint::Httplike, None, path) => (format!(""), format!("{path}")),
            (GitUrlParseHint::Sshlike, Some(port), path) => {
                (format!(":{port}"), format!("/{path}"))
            }
            (GitUrlParseHint::Sshlike, None, path) => (format!(""), format!(":{path}")),
            (GitUrlParseHint::Filelike, None, path) => (format!(""), format!("{path}")),
            _ => (format!(""), format!("")),
        };

        let git_url_str = format!("{scheme}{auth_info}{host}{port}{path}");

        write!(f, "{git_url_str}",)
    }
}

// This is to hide `url` from debug output
impl fmt::Debug for GitUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        #[derive(Debug)]
        struct GitUrl<'a> {
            scheme: Option<&'a str>,
            user: Option<&'a str>,
            token: Option<&'a str>,
            host: Option<&'a str>,
            port: Option<u16>,
            path: &'a str,
        }

        let Self {
            //url: _,
            scheme,
            user,
            token,
            host,
            port,
            path,
            print_scheme: _,
            hint: _,
        } = self;

        fmt::Debug::fmt(
            &GitUrl {
                scheme: *scheme,
                user: *user,
                token: *token,
                host: *host,
                port: *port,
                path: *path,
            },
            f,
        )
    }
}

impl<'url> GitUrl<'url> {
    /// Returns `GitUrl` after removing `user` and `token` values
    /// Intended use-case is for non-destructive printing GitUrl excluding any embedded auth info
    pub fn trim_auth(&self) -> GitUrl {
        let mut new_giturl = self.clone();
        new_giturl.set_user(None);
        new_giturl.set_token(None);
        new_giturl
    }

    // https://datatracker.ietf.org/doc/html/rfc3986
    // Based on rfc3986, but does not strictly cover the spec
    // * No support for:
    //     * query, fragment, percent-encoding, and much of the edges for path support
    //     * many forms of ip representations like ipv6, hexdigits
    // * Added support for:
    //     * parsing ssh git urls which use ":" as a delimiter between the authority and path
    //     * parsing userinfo into user:token (but its officially deprecated, per #section-3.2.1)
    //     * some limited support for windows/linux filepaths
    pub fn parse(input: &'url str) -> Result<Self, GitUrlParseError> {
        // Error if there are null bytes within the url
        // https://github.com/tjtelan/git-url-parse-rs/issues/16
        if input.contains('\0') {
            return Err(GitUrlParseError::FoundNullBytes);
        }

        //let original = input.to_string();

        let (input, mut scheme) = Self::parse_scheme.parse(input).finish().unwrap_or_default();
        let (_input, heir_part) = Self::parse_hier_part(input).finish().unwrap_or_default();

        let (user_opt, token_opt) = heir_part.0.0;
        let host_opt = heir_part.0.1;
        let port_opt = heir_part.0.2;
        let mut path = heir_part.1;

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
        } else {
            if user_opt.is_none()
                && token_opt.is_none()
                && host_opt.is_none()
                && port_opt.is_none()
                && !path.is_empty()
            {
                // if we only have a path => file
                GitUrlParseHint::Filelike
            } else if user_opt.is_some() && token_opt.is_some() {
                // If we have a user and token => http
                GitUrlParseHint::Httplike
            } else if path.starts_with(':') {
                // If path starts with a colon => ssh
                //if path.starts_with(':') {
                GitUrlParseHint::Sshlike
                //} else {
                //    GitUrlParseHint::Unknown
                //}
            } else {
                GitUrlParseHint::Unknown
            }
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
            user: user_opt,
            token: token_opt,
            host: host_opt,
            port: port_opt,
            path,
            //url: original,
            print_scheme,
            hint,
        };

        if git_url.is_valid() {
            Ok(git_url)
        } else {
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }

    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
    where
        T: GitProvider<GitUrl<'url>, GitUrlParseError>,
    {
        T::from_git_url(self)
    }

    fn is_valid(&self) -> bool {
        // Last chance validation

        //println!("{self:#?}");

        // There's an edge case we don't cover: ssh urls using ports + absolute paths
        // https://mslinn.com/git/040-git-urls.html - describes this pattern, if we decide to parse for it

        // only ssh paths start with ':'
        if self.hint() != GitUrlParseHint::Sshlike {
            if self.path.starts_with(':') {
                return false;
            }
        }

        // if we are not httplike, we shouldn't have tokens
        if self.hint() != GitUrlParseHint::Httplike {
            if self.token().is_some() {
                return false;
            }
        }

        // if we are filelike, we should only have paths
        if self.hint() == GitUrlParseHint::Filelike {
            if self.user().is_some()
                || self.token().is_some()
                || self.host().is_some()
                || self.port().is_some()
                || self.path().is_empty()
            {
                return false;
            }
        }

        true
    }

    fn parse_scheme(input: &'url str) -> IResult<&'url str, Option<&'url str>> {
        let mut check = context(
            "scheme validate",
            peek(pair(
                pair(
                    alpha1,
                    take_while(|c: char| {
                        c.is_ascii_alphabetic()
                            || c.is_ascii_digit()
                            || c == '+'
                            || c == '-'
                            || c == '.'
                    }),
                ),
                tag::<&str, &str, nom::error::Error<&str>>("://"),
            )),
        );

        if check.parse(input).is_err() {
            return Ok((input, None));
        }

        // Must start with alpha character, then alpha/digit/+/-/.
        context(
            "Scheme parse",
            opt(verify(
                terminated(
                    recognize(pair(
                        alpha1,
                        take_while(|c: char| {
                            c.is_ascii_alphabetic()
                                || c.is_ascii_digit()
                                || c == '+'
                                || c == '-'
                                || c == '.'
                        }),
                    )),
                    // Not part of spec. We consume the "://" here to more easily manage scheme to be optional
                    tag("://"),
                ),
                |s: &str| !s.is_empty(),
            )),
        )
        .parse(input)
    }

    // https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
    // The rfc says parsing the "//" part of the uri belongs to the hier-part parsing
    // but we only support common internet protocols, file paths, but not other "baseless" ones
    // so it is sensible for this move it with scheme parsing to support git user service urls
    fn parse_hier_part(
        input: &'url str,
    ) -> IResult<
        &'url str,
        (
            ((Option<&str>, Option<&str>), Option<&str>, Option<u16>),
            &'url str,
        ),
    > {
        let (input, authority) = Self::parse_authority(input)?;
        //println!("authority: {authority:?}");

        let (input, part) = context(
            "Top of path parsers",
            verify(
                alt((
                    //preceded(tag("//"), Self::path_abempty_parser()),
                    Self::path_abempty_parser(),
                    Self::path_rootless_parser(),
                    Self::path_ssh_parser(),
                )),
                |s: &str| !s.is_empty(),
            ),
        )
        .parse(input)?;

        Ok((input, (authority, part)))
    }

    fn parse_authority(
        input: &'url str,
    ) -> IResult<&'url str, ((Option<&str>, Option<&str>), Option<&str>, Option<u16>)> {
        // Optional: username / token
        let (input, userinfo) = Self::parse_userinfo(input)?;

        // Host

        // peek ahead to check for windows path stuff
        let check = context(
            "Host check for windows path",
            peek(preceded(
                take_while(|c| reg_name_uri_chars(c) && c != '\\'),
                tag::<&str, &str, nom::error::Error<&str>>(":\\"),
            )),
        )
        .parse(input);

        if check.is_ok() {
            return Ok((input, (userinfo, None, None)));
        }

        let (input, host) = context(
            "Host parser",
            opt(verify(
                recognize(take_while(|c: char| reg_name_uri_chars(c))),
                |s: &str| {
                    let has_alphanum = s.chars().into_iter().find(|c| is_alphanum(*c)).is_some();
                    let starts_with_alphanum = s.chars().next().is_some_and(|c| is_alphanum(c));

                    has_alphanum && starts_with_alphanum && !s.is_empty()
                },
            )),
        )
        .parse(input)?;

        // Optional: port
        let (input, port) = Self::parse_port(input)?;

        Ok((input, (userinfo, host, port)))
    }

    fn parse_userinfo(
        authority_input: &'url str,
    ) -> IResult<&'url str, (Option<&'url str>, Option<&'url str>)> {
        // Peek for username@
        let mut check = context(
            "Userinfo validation",
            peek(pair(
                take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'),
                tag::<&str, &str, nom::error::Error<&str>>("@"),
            )),
        );

        if check.parse(authority_input).is_err() {
            return Ok((authority_input, (None, None)));
        }

        // Userinfo
        let (authority_input, userinfo) = context(
            "Userinfo parser",
            opt(verify(
                recognize(take_while(|c: char| {
                    unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'
                })),
                |s: &str| !s.is_empty(),
            )),
        )
        .parse(authority_input)?;

        let (authority_input, _) = if userinfo.is_some() {
            context("Userinfo '@' parser", tag("@")).parse(authority_input)?
        } else {
            // No change to input, but let the compiler be happy
            (authority_input, authority_input)
        };

        // Break down userinfo into user and token
        let (user, token) = if let Some(userinfo) = userinfo {
            if userinfo.contains(":") {
                let (_, (user, token)) = context(
                    "Userinfo with colon parser",
                    separated_pair(
                        verify(
                            take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                            |s: &str| !s.is_empty(),
                        ),
                        tag(":"),
                        verify(
                            take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                            |s: &str| !s.is_empty(),
                        ),
                    ),
                )
                .parse(userinfo)?;
                (Some(user), Some(token))
            } else {
                (Some(userinfo), None)
            }
        } else {
            (None, None)
        };

        Ok((authority_input, (user, token)))
    }

    fn parse_port(authority_input: &'url str) -> IResult<&'url str, Option<u16>> {
        context(
            "Port parser",
            opt(map_opt(
                verify(preceded(tag(":"), digit1), |p_str: &str| !p_str.is_empty()),
                |s: &str| s.parse::<u16>().ok(),
            )),
        )
        .parse(authority_input)
    }

    // This will get absolute paths.
    // todo: test for empty and start with "//"
    fn path_abempty_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        // Starts with '/' or empty
        context(
            "Path parser (abempty)",
            recognize(many1(pair(
                tag("/"),
                take_while(|c: char| pchar_uri_chars(c)),
            ))),
        )
    }

    fn path_ssh_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        context(
            "Path parser (ssh)",
            recognize((
                tag(":"),
                take_while(|c: char| pchar_uri_chars(c)),
                many1(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
            )),
        )
    }

    fn path_rootless_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        context(
            "Path parser (rootless)",
            recognize(pair(
                take_while(|c: char| pchar_uri_chars(c)),
                many0(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
            )),
        )
    }
}

fn pchar_uri_chars(c: char) -> bool {
    // unreserved / pct-encoded (not implemented) / sub-delims / ":" / "@"
    unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':' || c == '@'
}

fn reg_name_uri_chars(c: char) -> bool {
    // *( unreserved / pct-encoded (not implemented) / sub-delims )
    unreserved_uri_chars(c) || subdelims_uri_chars(c)
}
fn unreserved_uri_chars(c: char) -> bool {
    is_alphanum(c) || c == '-' || c == '.' || c == '_' || c == '~'
}

fn is_alphanum(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit()
}

fn subdelims_uri_chars(c: char) -> bool {
    c == '!'
        || c == '$'
        || c == '&'
        || c == '\''
        || c == '('
        || c == ')'
        || c == '*'
        || c == '+'
        || c == ','
        || c == ';'
        || c == '='
        || c == '\\' // This is not part of spec, but used for windows paths
}
