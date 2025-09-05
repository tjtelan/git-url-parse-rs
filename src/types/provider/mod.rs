use nom::bytes::complete::{is_not, tag};
use nom::sequence::separated_pair;
use nom::{IResult, Parser, combinator::opt, combinator::rest};

use derive_builder::Builder;
use getset::{Getters, Setters};

use crate::{GitUrl, GitUrlParseError};

pub trait GitProvider<T, E>: Clone + std::fmt::Debug {
    fn from_git_url(url: &T) -> Result<Self, E>;
}

// todo: builder and setters be private?
#[derive(Debug, PartialEq, Eq, Clone, Builder, Default, Getters, Setters)]
pub struct GenericProvider {
    pub host: String,
    pub owner: String,
    pub repo: String,
}
impl GenericProvider {
    fn _get_owner_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (input, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(input)
    }

    // todo
    pub fn fullname(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl GitProvider<GitUrl<'_>, GitUrlParseError> for GenericProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Some(path), Some(host)) = (url.path(), url.host()) {
            if let Ok((_, Some((user, repo)))) = GenericProvider::_get_owner_repo(path) {
                Ok(GenericProvider {
                    host: host.to_string(),
                    owner: user.to_string(),
                    repo: repo.to_string(),
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
pub struct AzureDevOpsProvider {
    pub host: String,
    pub org: String,
    pub project: String,
    pub repo: String,
}
impl AzureDevOpsProvider {
    fn _get_user_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (n, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(n)
    }
}

impl GitProvider<GitUrl<'_>, GitUrlParseError> for AzureDevOpsProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Some(path), Some(host)) = (url.path(), url.host()) {
            if let Ok((_, Some((user, repo)))) = AzureDevOpsProvider::_get_user_repo(path) {
                Ok(AzureDevOpsProvider {
                    host: host.to_string(),
                    org: String::from(""),
                    project: String::from(user),
                    repo: String::from(repo),
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
pub struct GitLabProvider {
    pub host: String,
    pub user: String,
    pub subgroup: Option<Vec<String>>,
    pub repo: String,
}
impl GitLabProvider {
    fn _get_user_repo(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        let (n, _) = opt(tag("/")).parse(input)?;
        opt(separated_pair(is_not("/"), tag("/"), rest)).parse(n)
    }
}

impl GitProvider<GitUrl<'_>, GitUrlParseError> for GitLabProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Some(path), Some(host)) = (url.path(), url.host()) {
            if let Ok((_, Some((user, repo)))) = GitLabProvider::_get_user_repo(path) {
                Ok(GitLabProvider {
                    host: host.to_string(),
                    user: String::from(""),
                    subgroup: None,
                    repo: String::from(repo),
                })
            } else {
                Err(GitUrlParseError::UnexpectedFormat)
            }
        } else {
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}
