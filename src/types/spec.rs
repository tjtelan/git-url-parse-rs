//! # GitUrl url spec parser
//!
//! Internal structs with RFC 3968 parsing logic for Git urls
//!

use getset::CopyGetters;
#[cfg(feature = "log")]
use log::debug;
use nom::Finish;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::alpha1;
use nom::combinator::{map_opt, peek, recognize, verify};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt};

/// Top-level struct for RFC 3986 spec parser
#[derive(Debug, Default, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub(crate) struct UrlSpecParser<'url> {
    /// RFC 3986 scheme
    pub(crate) scheme: Option<&'url str>,
    /// RFC 3986 hier-part
    pub(crate) hier_part: UrlHierPart<'url>,
}

impl<'url> UrlSpecParser<'url> {
    /// https://datatracker.ietf.org/doc/html/rfc3986
    /// Based on rfc3986, but does not strictly cover the spec
    /// * No support for:
    ///     * query, fragment, percent-encoding, and much of the edges for path support
    ///     * many forms of ip representations like ipv6, hexdigits
    /// * Added support for:
    ///     * parsing ssh git urls which use ":" as a delimiter between the authority and path
    ///     * parsing userinfo into user:token (but its officially deprecated, per #section-3.2.1)
    ///     * some limited support for windows/linux filepaths
    pub(crate) fn parse(input: &'url str) -> IResult<&'url str, Self> {
        let (input, scheme) = Self::parse_scheme.parse(input).finish().unwrap_or_default();
        let (input, heir_part) = Self::parse_hier_part(input).finish().unwrap_or_default();

        let parsed = UrlSpecParser {
            scheme,
            hier_part: heir_part,
        };

        Ok((input, parsed))
    }

    /// RFC 3986 scheme
    fn parse_scheme(input: &'url str) -> IResult<&'url str, Option<&'url str>> {
        #[cfg(feature = "log")]
        {
            debug!("Looking ahead before parsing for scheme");
        }

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
            #[cfg(feature = "log")]
            {
                debug!("Look ahead check for scheme failed");
            }

            return Ok((input, None));
        }

        #[cfg(feature = "log")]
        {
            debug!("Look ahead check passed, parsing for scheme");
        }

        // Must start with alpha character, then alpha/digit/+/-/.
        let (input, scheme) = context(
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
        .parse(input)?;

        #[cfg(feature = "log")]
        {
            debug!("{input:?}");
            debug!("{scheme:?}");
        }

        Ok((input, scheme))
    }

    /// RFC 3986 hier-part
    // https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
    // The rfc says parsing the "//" part of the uri belongs to the hier-part parsing
    // but we only support common internet protocols, file paths, but not other "baseless" ones
    // so it is sensible for this move it with scheme parsing to support git user service urls
    fn parse_hier_part(input: &'url str) -> IResult<&'url str, UrlHierPart<'url>> {
        #[cfg(feature = "log")]
        {
            debug!("Parsing for heir-part");
        }

        let (input, authority) = Self::parse_authority(input)?;

        let (input, path) = context(
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

        let hier_part = UrlHierPart { authority, path };

        #[cfg(feature = "log")]
        {
            debug!("{:?}", input);
            debug!("{:?}", hier_part);
        }

        Ok((input, hier_part))
    }

    /// RFC 3986 authority
    fn parse_authority(input: &'url str) -> IResult<&'url str, UrlAuthority<'url>> {
        #[cfg(feature = "log")]
        {
            debug!("Parsing for Authority");
        }

        // Optional: username / token
        let (input, userinfo) = Self::parse_userinfo(input)?;

        // Host
        #[cfg(feature = "log")]
        {
            debug!("Looking ahead for windows-style path vs host");
        }

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
            #[cfg(feature = "log")]
            {
                debug!(
                    "Host check failed. Found potential windows-style path while looking for host"
                );
            }

            return Ok((input, UrlAuthority::default()));
        }

        #[cfg(feature = "log")]
        {
            debug!("Parsing for host");
        }

        let (input, host) = context(
            "Host parser",
            opt(verify(
                recognize(take_while(|c: char| reg_name_uri_chars(c))),
                |s: &str| {
                    let has_alphanum = s.chars().any(char::is_alphanumeric);
                    let starts_with_alphanum = s.chars().next().is_some_and(char::is_alphanumeric);

                    has_alphanum && starts_with_alphanum && !s.is_empty()
                },
            )),
        )
        .parse(input)?;

        #[cfg(feature = "log")]
        {
            debug!("host found: {host:?}");
        }

        // Optional: port
        let (input, port) = Self::parse_port(input)?;

        let authority = UrlAuthority {
            userinfo,
            host,
            port,
        };

        #[cfg(feature = "log")]
        {
            debug!("{input:?}");
            debug!("{authority:?}");
        }

        Ok((input, authority))
    }

    /// RFC 3986 userinfo
    fn parse_userinfo(authority_input: &'url str) -> IResult<&'url str, UrlUserInfo<'url>> {
        // Peek for username@
        #[cfg(feature = "log")]
        {
            debug!("Checking for for Userinfo");
        }

        let mut check = context(
            "Userinfo validation",
            peek(pair(
                take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'),
                tag::<&str, &str, nom::error::Error<&str>>("@"),
            )),
        );

        if check.parse(authority_input).is_err() {
            #[cfg(feature = "log")]
            {
                debug!("Userinfo check failed");
            }
            return Ok((authority_input, UrlUserInfo::default()));
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
            #[cfg(feature = "log")]
            {
                debug!("Userinfo found. Parsing for '@'");
            }

            context("Userinfo '@' parser", tag("@")).parse(authority_input)?
        } else {
            // No change to input, but let the compiler be happy
            (authority_input, authority_input)
        };

        // Break down userinfo into user and token
        let (user, token) = if let Some(userinfo) = userinfo {
            if userinfo.contains(":") {
                #[cfg(feature = "log")]
                {
                    debug!("Continue break down userinfo into user:token");
                }
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

        let userinfo = UrlUserInfo { user, token };

        #[cfg(feature = "log")]
        {
            debug!("{authority_input:?}");
            debug!("{userinfo:?}");
        }

        Ok((authority_input, userinfo))
    }

    /// RFC 3986 port
    fn parse_port(authority_input: &'url str) -> IResult<&'url str, Option<u16>> {
        #[cfg(feature = "log")]
        {
            debug!("Parsing port");
        }

        // We need to pull the full value of what's in the segment THEN parse for numbers
        let (input, port) = context(
            "Port parser",
            opt(map_opt(
                verify(
                    preceded(
                        tag(":"),
                        take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                    ),
                    |p_str: &str| !p_str.is_empty(),
                ),
                |s: &str| s.parse::<u16>().ok(),
            )),
        )
        .parse(authority_input)?;

        #[cfg(feature = "log")]
        {
            debug!("{authority_input:?}");
            debug!("{port:?}");
        }

        Ok((input, port))
    }

    /// RFC 3986 path-abempty
    fn path_abempty_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        #[cfg(feature = "log")]
        {
            debug!("parsing abempty path");
        }

        // Starts with '/' or empty
        context(
            "Path parser (abempty)",
            recognize(many1(pair(
                tag("/"),
                take_while(|c: char| pchar_uri_chars(c)),
            ))),
        )
    }

    /// Not part of RFC 3986 - ssh-based url path
    fn path_ssh_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        #[cfg(feature = "log")]
        {
            debug!("Parsing ssh path");
        }

        context(
            "Path parser (ssh)",
            recognize((
                tag(":"),
                take_while(|c: char| pchar_uri_chars(c)),
                many1(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
            )),
        )
    }

    /// RFC 3986 path-rootless
    fn path_rootless_parser(
    ) -> impl Parser<
        &'url str,
        Output = <dyn Parser<&'url str, Output = &'url str, Error = nom::error::Error<&'url str>> as Parser<
            &'url str,
        >>::Output,
        Error = nom::error::Error<&'url str>,
    >{
        #[cfg(feature = "log")]
        {
            debug!("Parsing rootless path");
        }

        context(
            "Path parser (rootless)",
            recognize(pair(
                take_while(|c: char| pchar_uri_chars(c)),
                many0(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
            )),
        )
    }
}

/// RFC 3986 userinfo
#[derive(Debug, Default, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub(crate) struct UrlUserInfo<'url> {
    /// RFC 3986 Userinfo
    pub(crate) user: Option<&'url str>,
    /// Non-spec, deprecated
    pub(crate) token: Option<&'url str>,
}

/// RFC 3986 authority
#[derive(Debug, Default, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub(crate) struct UrlAuthority<'url> {
    /// RFC 3986 Username, non-spec token
    pub(crate) userinfo: UrlUserInfo<'url>,
    /// RFC 3986 Host
    pub(crate) host: Option<&'url str>,
    /// RFC 3986 Port
    pub(crate) port: Option<u16>,
}

/// RFC 3986 hier-part
#[derive(Debug, Default, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub(crate) struct UrlHierPart<'url> {
    /// RFC 3986 authority
    pub(crate) authority: UrlAuthority<'url>,
    /// RFC 3986 relative-part
    pub(crate) path: &'url str,
}

/// RFC 3986 pchar
pub(crate) fn pchar_uri_chars(c: char) -> bool {
    // unreserved / pct-encoded (not implemented) / sub-delims / ":" / "@"
    unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':' || c == '@'
}

/// RFC 3986 reg-name
pub(crate) fn reg_name_uri_chars(c: char) -> bool {
    // *( unreserved / pct-encoded (not implemented) / sub-delims )
    unreserved_uri_chars(c) || subdelims_uri_chars(c)
}

/// RFC 3986 unreserved
pub(crate) fn unreserved_uri_chars(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~'
}

/// RFC 3986 sub-delims (mostly)
pub(crate) fn subdelims_uri_chars(c: char) -> bool {
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
