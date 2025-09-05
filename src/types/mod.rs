mod error;
mod provider;

pub use error::GitUrlParseError;
pub use provider::{AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider};

use core::str;
use std::fmt;
use std::str::FromStr;

use derive_builder::Builder;
use getset::{CloneGetters, CopyGetters, Getters, Setters};
use nom::Finish;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::one_of;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{peek, recognize, verify};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt, combinator::rest};

use strum::{Display, EnumString, VariantNames};
#[cfg(feature = "tracing")]
use tracing::debug;
use typed_path::{Utf8TypedPath, Utf8TypedPathBuf};

//// todo: let's get rid of this
///// Supported uri schemes for parsing
//#[derive(Debug, PartialEq, Eq, EnumString, VariantNames, Clone, Display)]
//#[strum(serialize_all = "kebab_case")]
//pub(crate) enum Scheme {
//    /// Represents `file://` url scheme
//    File,
//    /// Represents `ftp://` url scheme
//    Ftp,
//    /// Represents `ftps://` url scheme
//    Ftps,
//    /// Represents `git://` url scheme
//    Git,
//    /// Represents `git+ssh://` url scheme
//    #[strum(serialize = "git+ssh")]
//    GitSsh,
//    /// Represents `http://` url scheme
//    Http,
//    /// Represents `https://` url scheme
//    Https,
//    /// Represents `ssh://` url scheme
//    Ssh,
//    ///// Represents No url scheme
//    //Unspecified,
//    Other(String), // todo: need test for this
//}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum GitUrlParseHint {
    #[default]
    Unknown,
    Sshlike,
    Filelike,
    Httplike,
}

///// GitUrl represents an input url that is a url used by git
///// Internally during parsing the url is sanitized and uses the `url` crate to perform
///// the majority of the parsing effort, and with some extra handling to expose
///// metadata used my many git hosting services
//#[derive(Debug, PartialEq, Eq, Clone, Builder, Getters, Setters)]
//#[builder(build_fn(validate = "Self::prebuild_check"), field(public))]
//#[get = "pub"]
//pub struct GitUrlOld<P = GenericProvider>
//where
//    P: GitProvider<GitUrlOld, GitUrlParseError>,
//{
//    /// The host, domain or IP of the repo
//    #[builder(setter(into, strip_option), default)]
//    host: Option<String>,
//    /// The url scheme
//    #[builder(setter(into, strip_option), default)]
//    scheme: Option<Scheme>,
//    /// Authentication user
//    #[builder(setter(into, strip_option), default)]
//    #[getset(set = "pub(crate)")]
//    user: Option<String>,
//    /// Authentication token (could appear in the https urls)
//    #[builder(setter(into, strip_option), default)]
//    #[getset(set = "pub(crate)")]
//    token: Option<String>,
//    /// The port where git service is hosted
//    #[builder(setter(into, strip_option), default)]
//    port: Option<u16>,
//    /// The path to repo w/ respect to user + hostname
//    #[builder(setter(into))]
//    path: Utf8TypedPathBuf,
//    /// Include scheme:// when printing url
//    #[builder(default)]
//    print_scheme: bool,
//    /// Hosted git provider info derived from GitUrl
//    #[builder(setter(into, strip_option), default)]
//    provider: Option<P>,
//}

//impl<P: GitProvider<GitUrlOld, GitUrlParseError>> GitUrlOldBuilder<P> {
//    pub fn trim_auth(&mut self) {
//        self.user = None;
//        self.token = None;
//    }
//
//    fn prebuild_check(&self) -> Result<(), String> {
//        //#[cfg(feature = "tracing")]
//        //debug!("Processing: {:?}", &url);
//
//        // Error if there are null bytes within the url
//
//        // https://github.com/tjtelan/git-url-parse-rs/issues/16
//        if let Some(Some(host)) = &self.host {
//            if host.contains('\0') {
//                return Err(GitUrlParseError::FoundNullBytes.to_string());
//            }
//
//            if host.is_empty() {
//                return Err(
//                    GitUrlParseError::UnexpectedEmptyValue(String::from("host")).to_string()
//                );
//            }
//        }
//
//        if let Some(Some(user)) = &self.user {
//            if user.contains('\0') {
//                return Err(GitUrlParseError::FoundNullBytes.to_string());
//            }
//
//            if user.is_empty() {
//                return Err(
//                    GitUrlParseError::UnexpectedEmptyValue(String::from("user")).to_string()
//                );
//            }
//        }
//
//        if let Some(Some(token)) = &self.token {
//            if token.contains('\0') {
//                return Err(GitUrlParseError::FoundNullBytes.to_string());
//            }
//
//            if token.is_empty() {
//                return Err(
//                    GitUrlParseError::UnexpectedEmptyValue(String::from("token")).to_string(),
//                );
//            }
//        }
//
//        if let Some(path) = &self.path {
//            if path.as_str().contains('\0') {
//                return Err(GitUrlParseError::FoundNullBytes.to_string());
//            }
//            if path.as_str().is_empty() {
//                return Err(
//                    GitUrlParseError::UnexpectedEmptyValue(String::from("path")).to_string()
//                );
//            }
//        }
//
//        Ok(())
//    }
//
//    fn parse(url: &str) -> Result<Self, GitUrlParseError> {
//        debug!("{url}");
//        let mut giturl = GitUrlOldBuilder::default();
//        let mut working_url = url;
//        let mut hint = GitUrlParseHint::default();
//
//        giturl.parse_scheme(&mut working_url, &mut hint);
//        giturl.parse_auth_info(&mut working_url, &mut hint);
//        let save_state = working_url;
//
//        giturl.parse_host_port(&mut working_url, &mut hint);
//
//        match hint {
//            GitUrlParseHint::Httplike => {
//                if working_url.starts_with(":") && giturl.port.is_none() {
//                    return Err(GitUrlParseError::UnexpectedFormat);
//                } else {
//                    println!("Nothing wrong here: {working_url}");
//                }
//            }
//            GitUrlParseHint::Sshlike => {
//                giturl.parse_ssh_path(&mut working_url, &mut hint);
//            }
//            GitUrlParseHint::Filelike | GitUrlParseHint::Unknown => {
//                working_url = save_state;
//                giturl.host = None;
//                giturl.scheme(Scheme::File);
//            }
//        }
//
//        giturl.parse_path(&mut working_url, &mut hint);
//
//        Ok(giturl)
//    }
//
//    fn parse_scheme(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
//        let mut builder = self.clone();
//
//        if let Ok((leftover, Some(s))) = GitUrlOldBuilder::<P>::_parse_scheme(working_url) {
//            println!("leftover: {leftover}, scheme: {s:?}");
//
//            let scheme = Scheme::from_str(s).expect("Unknown scheme");
//
//            *hint = match &scheme {
//                Scheme::Ssh => GitUrlParseHint::Sshlike,
//                Scheme::File => GitUrlParseHint::Filelike,
//                _ => GitUrlParseHint::Httplike,
//            };
//
//            builder.scheme(scheme);
//            builder.print_scheme(true);
//
//            *self = builder;
//            *working_url = leftover;
//        }
//    }
//
//    fn parse_auth_info(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
//        let mut builder = self.clone();
//        if let Ok((leftover, Some(username))) = GitUrlOldBuilder::<P>::_parse_username(working_url)
//        {
//            println!("leftover: {leftover}, username: {username:?}");
//            builder.user(username);
//
//            if *hint == GitUrlParseHint::Unknown {
//                *hint = GitUrlParseHint::Sshlike;
//            }
//
//            if let Ok((token, Some(real_username))) = GitUrlOldBuilder::<P>::_parse_token(username)
//            {
//                println!("token: {token}, real_username: {real_username:?}");
//                builder.user(real_username);
//                builder.token(token);
//
//                if *hint == GitUrlParseHint::Unknown || *hint == GitUrlParseHint::Sshlike {
//                    *hint = GitUrlParseHint::Httplike;
//                }
//            }
//
//            *working_url = leftover;
//            *self = builder;
//        }
//    }
//
//    fn parse_host_port(&mut self, working_url: &mut &str, hint: &mut GitUrlParseHint) {
//        let mut builder = self.clone();
//        let mut save = working_url.clone();
//
//        if let Ok((leftover, Some(hostname))) = GitUrlOldBuilder::<P>::_parse_hostname(save) {
//            println!("leftover {leftover}, hostname: {hostname}");
//            builder.host(hostname);
//            save = leftover;
//        }
//
//        if let Ok((leftover, Some(port))) = GitUrlOldBuilder::<P>::_parse_port(save) {
//            if !port.is_empty() {
//                println!("leftover {leftover}, port: {port}");
//                builder.port(u16::from_str(port).expect("Not a valid port"));
//                save = leftover;
//
//                // If we're currently uncertain, but we've found a port
//                // our guess is this more likely is an http url than an ssh url
//                // Add the `ssh://` scheme to the url if this is incorrect
//                if *hint == GitUrlParseHint::Unknown {
//                    *hint = GitUrlParseHint::Httplike;
//                }
//            }
//        }
//
//        // https://mslinn.com/git/040-git-urls.html - we only support relative paths when we have ports
//        if builder.port.is_none() && save.starts_with(":") {
//            *hint = GitUrlParseHint::Sshlike;
//        }
//
//        *self = builder;
//        *working_url = save;
//    }
//
//    fn parse_ssh_path(&mut self, working_url: &mut &str, _hint: &mut GitUrlParseHint) {
//        let mut builder = self.clone();
//
//        if let Ok((_leftover, Some(path))) = GitUrlOldBuilder::<P>::_parse_ssh_path(working_url) {
//            builder.scheme(Scheme::Ssh);
//
//            *self = builder;
//            *working_url = path;
//        }
//    }
//
//    fn parse_path(&mut self, working_url: &mut &str, _hint: &mut GitUrlParseHint) {
//        let mut builder = self.clone();
//        if let Ok((leftover, path)) = GitUrlOldBuilder::<P>::_parse_path(working_url) {
//            println!("leftover {leftover}, path: {path}");
//
//            let parsed_path = Utf8TypedPath::derive(path).to_path_buf();
//            builder.path(parsed_path);
//
//            *self = builder;
//            *working_url = leftover;
//        }
//    }
//
//    ////
//
//    fn _parse_scheme(input: &str) -> IResult<&str, Option<&str>> {
//        opt(terminated(
//            alt((
//                // Fancy: Can I build an iter map on this?
//                tag(Scheme::File.to_string().as_bytes()),
//                tag(Scheme::Ftps.to_string().as_bytes()),
//                tag(Scheme::Ftp.to_string().as_bytes()),
//                tag(Scheme::GitSsh.to_string().as_bytes()),
//                tag(Scheme::Git.to_string().as_bytes()),
//                tag(Scheme::Https.to_string().as_bytes()),
//                tag(Scheme::Http.to_string().as_bytes()),
//                tag(Scheme::Ssh.to_string().as_bytes()),
//                // todo: Other(), needs a test
//            )),
//            tag("://"),
//        ))
//        .parse(input)
//    }
//
//    fn _parse_username(input: &str) -> IResult<&str, Option<&str>> {
//        opt(terminated(take_until("@"), tag("@"))).parse(input)
//    }
//
//    fn _parse_token(input: &str) -> IResult<&str, Option<&str>> {
//        opt(terminated(take_until(":"), tag(":"))).parse(input)
//    }
//
//    fn _parse_hostname(input: &str) -> IResult<&str, Option<&str>> {
//        opt(take_till(|c| c == '/' || c == ':')).parse(input)
//    }
//
//    fn _parse_port(input: &str) -> IResult<&str, Option<&str>> {
//        opt(preceded(tag(":"), take_while(|c: char| c.is_ascii_digit()))).parse(input)
//    }
//
//    // This is making an assumption that the path is relative, not absolute
//    // This is bc we do not support absolute paths when we also have a port
//    fn _parse_ssh_path(input: &str) -> IResult<&str, Option<&str>> {
//        opt(preceded(one_of("/:"), rest)).parse(input)
//    }
//
//    fn _parse_path(input: &str) -> IResult<&str, &str> {
//        rest(input)
//    }
//}

// TODO: Revisit this
/// Build the printable GitUrl from its components
impl fmt::Display for GitUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scheme = if let Some(scheme) = &self.scheme()
            && self.print_scheme()
        {
            format!("{scheme}://")
        } else {
            String::new()
        };

        let auth_info = match self.scheme() {
            Some("ssh") | Some("git") | Some("git+ssh") => {
                if let Some(user) = &self.user() {
                    format!("{user}@")
                } else {
                    String::new()
                }
            }
            Some("http") | Some("https") => match (&self.user(), &self.token()) {
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
            Some(p) => format!(":{p}",),
            None => String::new(),
        };

        let path = if let Some(path) = &self.path() {
            if self.scheme().clone() == Some("ssh") {
                if self.port().is_some() {
                    if !path.starts_with('/') {
                        format!("/{path}")
                    } else {
                        path.to_string()
                    }
                } else {
                    format!(":{path}")
                }
            } else {
                path.to_string()
            }
        } else {
            String::new()
        };

        let git_url_str = format!("{scheme}{auth_info}{host}{port}{path}");

        write!(f, "{git_url_str}",)
    }
}

//impl FromStr for GitUrlOld {
//    //type Err = GitUrlParseError;
//    type Err = GitUrlOldBuilderError;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        GitUrlOld::parse(s)
//    }
//}
//
//impl GitUrlOld {
//    /// Returns `GitUrl` after removing `user` and `token` values
//    /// Intended use-case is for non-destructive printing GitUrl excluding any embedded auth info
//    pub fn trim_auth(&self) -> GitUrlOld {
//        let mut new_giturl = self.clone();
//        new_giturl.set_user(None);
//        new_giturl.set_token(None);
//        new_giturl
//    }
//
//    /// Returns a `Result<GitUrl>` after normalizing and parsing `url` for metadata
//    pub fn parse(url: &str) -> Result<GitUrlOld, GitUrlOldBuilderError> {
//        let giturl = GitUrlOldBuilder::parse(url).unwrap();
//        giturl.build()
//    }
//
//    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
//    where
//        T: GitProvider<GitUrlOld, GitUrlParseError>,
//    {
//        T::from_git_url(self)
//    }
//}

#[derive(Clone, Debug, CopyGetters, CloneGetters, Setters, Default, PartialEq, Eq)]
//pub struct GitUrl<'a, P = GenericProvider>
pub struct GitUrl<'a>
//where
//    P: GitProvider<GitUrl<'a>, GitUrlParseError>,
{
    #[getset(get_clone = "pub", set = "pub(crate)")]
    url: String,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    scheme: Option<&'a str>,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    user: Option<&'a str>,
    #[getset(get_copy = "pub", set = "pub(crate)")]
    token: Option<&'a str>,
    #[getset(get_copy = "pub")]
    host: Option<&'a str>,
    #[getset(get_copy = "pub")]
    port: Option<&'a str>,
    #[getset(get_copy = "pub")]
    path: Option<&'a str>,
    /// Include scheme:// when printing url
    #[getset(get_copy = "pub")]
    print_scheme: bool,
    ///// Hosted git provider info derived from GitUrl
    //#[getset(skip)]
    //provider: Option<P>,
}

impl<'a> GitUrl<'a> {
    pub fn provider_info<T>(&self) -> Result<T, GitUrlParseError>
    where
        T: GitProvider<GitUrl<'a>, GitUrlParseError>,
    {
        T::from_git_url(self)
    }

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
    pub fn parse(input: &'a str) -> Result<Self, GitUrlParseError> {
        // Error if there are null bytes within the url
        // https://github.com/tjtelan/git-url-parse-rs/issues/16
        if input.contains('\0') {
            return Err(GitUrlParseError::FoundNullBytes);
        }

        let original = input;

        let (input, scheme) = Self::parse_scheme.parse(input).finish().unwrap_or_default();
        let (_input, heir_part) = Self::parse_hier_part(input).finish().unwrap_or_default();

        let (user_opt, token_opt) = heir_part.0.0;
        let host_opt = heir_part.0.1;
        let port_opt = heir_part.0.2;
        let path_opt = heir_part.1;

        // This needs another pass
        let provider = if let Some(scheme) = scheme {
            if scheme == "http" || scheme == "https" || scheme == "ssh" {
                Some(GenericProvider::default())
            } else {
                None
            }
        } else {
            None
        };

        Ok(GitUrl {
            url: original.to_string(),
            scheme,
            user: user_opt,
            token: token_opt,
            host: host_opt,
            port: port_opt,
            path: path_opt,
            print_scheme: scheme.is_some(),
            //provider
        })
    }

    pub fn parse_scheme(input: &'a str) -> IResult<&'a str, Option<&'a str>> {
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
            opt(terminated(
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
            )),
        )
        .parse(input)
    }

    // https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
    // The rfc says parsing the "//" part of the uri belongs to the hier-part parsing
    // but we only support common internet protocols, file paths, but not other "baseless" ones
    // so it is sensible for this move it with scheme parsing to support git user service urls
    pub fn parse_hier_part(
        input: &'a str,
    ) -> IResult<
        &'a str,
        (
            ((Option<&str>, Option<&str>), Option<&str>, Option<&str>),
            Option<&'a str>,
        ),
    > {
        let (input, authority) = Self::parse_authority(input)?;
        //println!("authority: {authority:?}");

        let (input, part) = context(
            "Top of path parsers",
            alt((
                //preceded(tag("//"), Self::path_abempty_parser()),
                Self::path_abempty_parser(),
                Self::path_rootless_parser(),
                Self::path_ssh_parser(),
            )),
        )
        .parse(input)?;

        Ok((input, (authority, Some(part))))
    }

    pub fn parse_authority(
        input: &'a str,
    ) -> IResult<&'a str, ((Option<&str>, Option<&str>), Option<&str>, Option<&str>)> {
        let original = input;

        // Optional: username / token
        let (input, userinfo) = Self::parse_userinfo(input)?;

        // Host
        let (input, host) = context(
            "Host parser",
            opt(verify(
                recognize(take_while(|c: char| reg_name_uri_chars(c))),
                |s: &str| {
                    let has_alphanum = s.chars().into_iter().find(|c| is_alphanum(*c)).is_some();
                    let starts_with_alphanum = s.chars().next().is_some_and(|c| is_alphanum(c));

                    has_alphanum && starts_with_alphanum
                },
            )),
        )
        .parse(input)?;

        // Optional: port
        let (input, port) = Self::parse_port(input)?;

        Ok((input, (userinfo, host, port)))
    }

    pub fn parse_userinfo(
        authority_input: &'a str,
    ) -> IResult<&'a str, (Option<&'a str>, Option<&'a str>)> {
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
            opt(recognize(take_while(|c: char| {
                unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'
            }))),
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
                        take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                        tag(":"),
                        take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
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

    pub fn parse_port(authority_input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        context("Port parser", opt(preceded(tag(":"), digit1))).parse(authority_input)
    }

    // This will get absolute paths.
    // todo: test for empty and start with "//"
    pub fn path_abempty_parser(
    ) -> impl Parser<
        &'a str,
        Output = <dyn Parser<&'a str, Output = &'a str, Error = nom::error::Error<&'a str>> as Parser<
            &'a str,
        >>::Output,
        Error = nom::error::Error<&'a str>,
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

    pub fn path_ssh_parser(
    ) -> impl Parser<
        &'a str,
        Output = <dyn Parser<&'a str, Output = &'a str, Error = nom::error::Error<&'a str>> as Parser<
            &'a str,
        >>::Output,
        Error = nom::error::Error<&'a str>,
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

    pub fn path_rootless_parser(
    ) -> impl Parser<
        &'a str,
        Output = <dyn Parser<&'a str, Output = &'a str, Error = nom::error::Error<&'a str>> as Parser<
            &'a str,
        >>::Output,
        Error = nom::error::Error<&'a str>,
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
