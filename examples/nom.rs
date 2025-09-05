use getset::{Getters, Setters};
use git_url_parse::{GitUrl, GitUrlParseError};
use nom::FindSubstring;
use nom::bytes::complete::{is_a, take_while};
use nom::character::complete::{digit1, one_of};
use nom::combinator::{opt, peek, verify};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::recognize,
    sequence::{pair, separated_pair},
};

#[derive(Debug, Getters, Setters, Default)]
struct GitUrl2<'a> {
    url: String,
    scheme: Option<&'a str>,
    user: Option<&'a str>,
    token: Option<&'a str>,
    host: Option<&'a str>,
    port: Option<&'a str>,
    path: Option<&'a str>,
}

impl<'a> GitUrl2<'a> {
    // https://datatracker.ietf.org/doc/html/rfc3986
    // Based on rfc3986, but does not strictly cover the spec
    // * No support for:
    //     * query, fragment, percent-encoding, and much of the edges for path support
    //     * many forms of ip representations like ipv6, hexdigits
    // * Added support for:
    //     * parsing ssh git urls which use ":" as a delimiter between the authority and path
    //     * parsing userinfo into user:token (but its officially deprecated, per #section-3.2.1)
    //     * some limited support for windows/linux filepaths
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let original = input;

        let (input, scheme) = Self::parse_scheme.parse(input)?;
        let (input, heir_part) = Self::parse_hier_part(input)?;

        let (user_opt, token_opt) = heir_part.0.0;
        let (host_opt) = heir_part.0.1;
        let (port_opt) = heir_part.0.2;
        let (path_opt) = heir_part.1;

        Ok((
            input,
            GitUrl2 {
                url: original.to_string(),
                scheme,
                user: user_opt,
                token: token_opt,
                host: host_opt,
                port: port_opt,
                path: path_opt,
            },
        ))
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

fn main() -> Result<(), GitUrlParseError> {
    env_logger::init();

    let test_vec = vec![
        "https://github.com/tjtelan/git-url-parse-rs.git",
        "git@github.com:tjtelan/git-url-parse-rs.git",
        "git@hostname:22/path/to/repo.git",
        "ssh://git@github.com:22/asdf/asdf.git",
        "https://token:x-oauth-basic@host.xz/path/to/repo.git/",
        "https://x-token-auth:token@host.xz/path/to/repo.git/",
        "git+ssh://git@some-host.com/and-the-path/name",
        "git://some-host.com/and-the-path/name",
        "host.tld:user/project-name.git",
        "file:///path/to/repo.git/",
        "~/path/to/repo.git/",
        "./path/to/repo.git/",
        "./path/to/repo.git",
        "/path/to/repo.git",
        "../test_repo",
        "..\\test_repo",
        "git@ssh.dev.azure.com:v3/CompanyName/ProjectName/RepoName",
        "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName",
    ];

    for test_url in test_vec {
        //let parsed = GitUrl::parse(test_url).unwrap();
        ////println!("leftover:{leftover:#?}, output:{output:#?}");
        ////let parsed = GitUrl::parse(test_url)?;
        ////println!("Original: {}", test_url);
        //println!("Parsed:   {}", parsed);
        //println!("Parsed:   {:#?}", parsed);
        ////println!("{:?}\n", parsed);

        let parsed = GitUrl2::parse(test_url).unwrap();
        println!("{parsed:#?}");
        //println!("{:?}", parsed.parse());
        println!("");
    }
    Ok(())
}
