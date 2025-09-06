use nom::bytes::complete::{is_not, tag, take_until, take_while};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt, combinator::recognize, combinator::rest};

use derive_builder::Builder;
use getset::CopyGetters;

use crate::types::GitUrlParseHint;
use crate::{GitUrl, GitUrlParseError};

pub trait GitProvider<T, E>: Clone + std::fmt::Debug {
    fn from_git_url(url: &T) -> Result<Self, E>;
}

// todo: builder and setters be private?
#[derive(Debug, PartialEq, Eq, Clone, Builder, Default, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct GenericProvider<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}
impl<'a> GenericProvider<'a> {
    fn parse_path(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, _) = opt(tag("/")).parse(input)?;
        separated_pair(is_not("/"), tag("/"), take_until(".git")).parse(input)
    }

    pub fn fullname(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GenericProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        let path = (url.path());
        if let Ok((_, (user, repo))) = Self::parse_path(path) {
            Ok(GenericProvider { owner: user, repo })
        } else {
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}

// todo: builder, optional
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AzureDevOpsProvider<'a> {
    pub org: &'a str,
    pub project: &'a str,
    pub repo: &'a str,
}
impl<'a> AzureDevOpsProvider<'a> {
    fn _get_user_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (n, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(n)
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for AzureDevOpsProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        if let (path, Some(host)) = (url.path(), url.host()) {
            if let Ok((_, Some((user, repo)))) = AzureDevOpsProvider::_get_user_repo(path) {
                Ok(AzureDevOpsProvider {
                    org: "",
                    project: user,
                    repo: repo,
                })
            } else {
                Err(GitUrlParseError::UnexpectedFormat)
            }
        } else {
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}

// todo: builder, optional
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GitLabProvider<'a> {
    pub user: &'a str,
    pub subgroup: Option<Vec<&'a str>>,
    pub repo: &'a str,
}
impl<'a> GitLabProvider<'a> {
    fn _get_user_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (n, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(n)
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GitLabProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        if let (path, Some(host)) = (url.path(), url.host()) {
            if let Ok((_, Some((user, repo)))) = GitLabProvider::_get_user_repo(path) {
                Ok(GitLabProvider {
                    user: "",
                    subgroup: None,
                    repo: repo,
                })
            } else {
                Err(GitUrlParseError::UnexpectedFormat)
            }
        } else {
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}
