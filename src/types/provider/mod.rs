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

impl GitProvider<GitUrl, GitUrlParseError> for GenericProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Ok((_, Some((user, repo)))), Some(host)) = (
            GenericProvider::_get_owner_repo(url.path().as_str()),
            url.host(),
        ) {
            Ok(GenericProvider {
                host: host.clone(),
                owner: String::from(user),
                repo: String::from(repo),
            })
        } else {
            // TODO: Check this error type later
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

impl GitProvider<GitUrl, GitUrlParseError> for AzureDevOpsProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Ok((_, Some((user, repo)))), Some(host)) = (
            AzureDevOpsProvider::_get_user_repo(url.path().as_str()),
            url.host(),
        ) {
            Ok(AzureDevOpsProvider {
                host: host.clone(),
                org: String::from(""),
                project: String::from(user),
                repo: String::from(repo),
            })
        } else {
            // TODO: Check this error type later
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

impl GitProvider<GitUrl, GitUrlParseError> for GitLabProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if let (Ok((_, Some((_user, repo)))), Some(host)) = (
            GitLabProvider::_get_user_repo(url.path().as_str()),
            url.host(),
        ) {
            Ok(GitLabProvider {
                host: host.clone(),
                user: String::from(""),
                subgroup: None,
                repo: String::from(repo),
            })
        } else {
            // TODO: Check this error type later
            Err(GitUrlParseError::UnexpectedFormat)
        }
    }
}
