// generic
// gitlab (subgroups) style
// azure devops

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};
use nom::character::complete::{alphanumeric1, anychar, one_of};
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt, combinator::rest};

use crate::{GitUrl, GitUrlParseError};

pub trait GitProvider<T, E>: Clone + std::fmt::Debug {
    fn from_git_url(url: &T) -> Result<Self, E>;
    //fn get_url(&self, provider: &str
    //fn register(&self);
    //fn unregister(&self);o
    //fn to_obj(&self) -> Box<Self> {
    //    Box::new(self.clone())
    //}
}

// todo: builder
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenericProvider {
    pub host: String,
    pub user: String,
    pub repo: String,
}
impl GenericProvider {
    fn _get_user_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (n, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(n)
    }

    // fn _get_path_segment
}

impl GitProvider<GitUrl, GitUrlParseError> for GenericProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Ok((_, Some((user, repo)))), Some(host)) =
            (GenericProvider::_get_user_repo(url.path()), url.host())
        {
            Ok(GenericProvider {
                host: host.clone(),
                user: String::from(user),
                repo: String::from(repo),
            })
        } else {
            // TODO: Check this error type later
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}
