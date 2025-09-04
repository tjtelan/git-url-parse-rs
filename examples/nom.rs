use getset::{Getters, Setters};
use git_url_parse::{GitUrl, GitUrlParseError};
use nom::FindSubstring;
use nom::bits::complete::take;
use nom::bytes::complete::{is_a, take_while};
use nom::character::complete::{digit1, one_of};
use nom::combinator::{opt, peek};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{consumed, recognize},
    multi::many0_count,
    sequence::{pair, separated_pair},
};
use std::borrow::Cow;

#[derive(Debug, Getters, Setters, Default)]
struct GitUrl2<'a> {
    url: String,
    scheme: Option<&'a str>,
}

impl<'a> GitUrl2<'a> {
    pub fn new(url: &str) -> Self {
        GitUrl2 {
            url: String::from(url),
            ..Default::default()
        }
    }

    // https://datatracker.ietf.org/doc/html/rfc3986#appendix-A

    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let original = input;
        let (input, scheme) = Self::parse_scheme.parse(input)?;

        let scheme_slice = if let Some(scheme) = scheme {
            if let Some(index) = original.find_substring(scheme) {
                //println!("scheme slice: {}", &original[index..(index+scheme.len())]);
                Some(&original[index..(index + scheme.len())])
            } else {
                None
            }
        } else {
            None
        };

        // Eat the ':' when we have a scheme
        //let (input, scheme) = if scheme.is_some() {
        //    let (input, _) = tag(":")(input)?;
        //    //self.scheme = Cow::Borrowed(&scheme);
        //    (input, scheme)
        //} else {
        //    (input, None)
        //};

        println!("scheme: {scheme:?}");

        let (input, heir_part) = Self::parse_hier_part(scheme.is_some(), input)?;
        println!("heir_part: {heir_part:?}");

        Ok((
            input,
            GitUrl2 {
                url: original.to_string(),
                scheme: scheme_slice,
            },
        ))
    }

    pub fn parse_scheme(input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        let mut check = peek(pair(
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
        ));

        if check.parse(input).is_err() {
            return Ok((input, None));
        }

        // Must start with alpha character, then alpha/digit/+/-/.
        //let (input, scheme) = opt(recognize(pair(
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
                tag(":"),
            )),
        )
        //.parse(input)?;
        .parse(input)

        //Ok((input, scheme))
    }

    pub fn parse_hier_part(scheme: bool, input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        //let input = if scheme {
        //    let (input, _) = tag("//")(input)?;
        //    input
        //} else {
        //    input
        //};

        let (input, authority) = Self::parse_authority(input)?;
        println!("authority: {authority:?}");
        //let (input, part) = self.path_abempty(input);
        let (input, part) = alt((
            preceded(tag("//"), Self::path_abempty_parser()),
            Self::path_rootless_parser(),
            Self::path_ssh_parser(),
        ))
        .parse(input)?;
        //alt((self.path_ssh_parser(), self.path_abempty_parser())).parse(input)?;

        //          / path-absolute
        //          / path-rootless
        //          / path-empty

        Ok((input, Some(part)))
    }

    pub fn parse_authority(input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        let original = input;

        // Optional: username
        let (input, username) = Self::parse_userinfo(input)?;

        if let Some(userinfo) = username {
            if userinfo.contains(":") {
                let (_, (user, token)) = separated_pair(
                    take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                    tag(":"),
                    take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c)),
                )
                .parse(userinfo)?;
                println!("user: {user:?}");
                println!("token: {token:?}");
            } else {
                println!("user: {userinfo:?}");
            }
        }

        // Host
        let (input, authority) =
            opt(recognize(take_while(|c: char| reg_name_uri_chars(c)))).parse(input)?;

        // Verify if found host is more than symbols
        if let Some(host) = authority {
            let is_alphanum = host.chars().into_iter().find(|c| is_alphanum(*c)).is_some();
            if !is_alphanum {
                return Ok((original, None));
            }
        }

        // Optional: port
        let (input, port) = Self::parse_port(input)?;
        if let Some(port) = port {
            println!("port: {port:?}");
        }

        Ok((input, authority))
    }

    pub fn parse_userinfo(authority_input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        // Peek for username@
        let mut check = peek(pair(
            take_while(|c: char| unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'),
            tag::<&str, &str, nom::error::Error<&str>>("@"),
        ));

        if check.parse(authority_input).is_err() {
            return Ok((authority_input, None));
        }

        // Username
        let (authority_input, userinfo) = opt(recognize(take_while(|c: char| {
            unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':'
        })))
        .parse(authority_input)?;

        let (authority_input, _) = if userinfo.is_some() {
            tag("@")(authority_input)?
        } else {
            // No change to input, but let the compiler be happy
            (authority_input, authority_input)
        };

        // Should I parse token in here?

        Ok((authority_input, userinfo))
    }

    pub fn parse_port(authority_input: &'a str) -> IResult<&'a str, Option<&'a str>> {
        opt(preceded(tag(":"), digit1)).parse(authority_input)
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
        recognize(many1(pair(
            tag("/"),
            take_while(|c: char| pchar_uri_chars(c)),
        )))
    }

    pub fn path_ssh_parser(
    ) -> impl Parser<
        &'a str,
        Output = <dyn Parser<&'a str, Output = &'a str, Error = nom::error::Error<&'a str>> as Parser<
            &'a str,
        >>::Output,
        Error = nom::error::Error<&'a str>,
    >{
        recognize((
            tag(":"),
            take_while(|c: char| pchar_uri_chars(c)),
            many1(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
        ))
    }

    //pub fn path_absolute_parser<'a>(
    //    &self,
    //) -> impl Parser<
    //    &str,
    //    Output = <dyn Parser<&str, Output = &str, Error = nom::error::Error<&str>> as Parser<
    //        &str,
    //    >>::Output,
    //    Error = nom::error::Error<&str>,
    //> {
    //    // Starts with '/' but not "//"
    //    recognize(many1(pair(
    //        tag("/"),
    //        take_while(|c: char| pchar_uri_chars(c)),
    //    )))
    //}

    pub fn path_rootless_parser(
    ) -> impl Parser<
        &'a str,
        Output = <dyn Parser<&'a str, Output = &'a str, Error = nom::error::Error<&'a str>> as Parser<
            &'a str,
        >>::Output,
        Error = nom::error::Error<&'a str>,
    >{
        recognize(pair(
            take_while(|c: char| pchar_uri_chars(c)),
            many0(pair(tag("/"), take_while(|c: char| pchar_uri_chars(c)))),
        ))
    }
}

fn pchar_uri_chars(c: char) -> bool {
    // unreserved / pct-encoded (not implemented) / sub-delims / ":" / "@"
    unreserved_uri_chars(c) || subdelims_uri_chars(c) || c == ':' || c == '@'
}

fn reg_name_uri_chars(c: char) -> bool {
    // *( unreserved / pct-encoded / sub-delims )
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
        println!("{parsed:?}");
        //println!("{:?}", parsed.parse());
        println!("");
    }
    Ok(())
}
