use nom::bytes::complete::{is_not, tag, take_until, take_while};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::{IResult, Parser, combinator::opt, combinator::recognize, combinator::rest};

use derive_builder::Builder;
use getset::{CloneGetters, CopyGetters};

use crate::types::{GitUrlParseHint, is_alphanum, provider};
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
    fn parse_path(input: &str) -> Result<(&str, GenericProvider), GitUrlParseError> {
        let parse_result = || -> IResult<&str, GenericProvider> {
            let (input, _) = opt(tag("/")).parse(input)?;
            let (input, (user, repo)) =
                separated_pair(is_not("/"), tag("/"), take_until(".git")).parse(input)?;
            Ok((input, GenericProvider { owner: user, repo }))
        };

        parse_result().map_err(|e| match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                GitUrlParseError::NomParseError(err.to_string())
            }
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedFormat,
        })
    }

    pub fn fullname(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GenericProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        if url.hint() == GitUrlParseHint::Filelike {
            return Err(GitUrlParseError::ProviderUnsupported)    
        }

        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct AzureDevOpsProvider<'a> {
    pub org: &'a str,
    pub project: &'a str,
    pub repo: &'a str,
}
impl<'a> AzureDevOpsProvider<'a> {
    fn parse_http_path(input: &str) -> Result<(&str, AzureDevOpsProvider), GitUrlParseError> {
        let parse_result = || -> IResult<&str, AzureDevOpsProvider> {
            // Handle optional leading /
            let (input, _) = opt(tag("/")).parse(input)?;

            // Parse org/project/repo
            let (input, (org, (project, repo))) = separated_pair(
                is_not("/"),
                tag("/"),
                separated_pair(
                    is_not("/"),
                    tag("/"),
                    preceded(opt(tag("_git/")), is_not("")),
                ),
            )
            .parse(input)?;

            Ok((input, AzureDevOpsProvider { org, project, repo }))
        };

        parse_result().map_err(|e| match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                GitUrlParseError::NomParseError(err.to_string())
            }
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedFormat,
        })
    }
    fn parse_ssh_path(input: &str) -> Result<(&str, AzureDevOpsProvider), GitUrlParseError> {
        let parse_result = || -> IResult<&str, AzureDevOpsProvider> {
            // Handle optional leading v3/ or other prefix
            let (input, _) = opt(take_until("/")).parse(input)?;
            let (input, _) = opt(tag("/")).parse(input)?;

            // Parse org/project/repo
            let (input, (org, (project, repo))) = separated_pair(
                is_not("/"),
                tag("/"),
                separated_pair(
                    is_not("/"),
                    tag("/"),
                    terminated(is_not("."), opt(tag(".git"))),
                ),
            )
            .parse(input)?;

            Ok((input, AzureDevOpsProvider { org, project, repo }))
        };

        parse_result().map_err(|e| match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                GitUrlParseError::NomParseError(err.to_string())
            }
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedFormat,
        })
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for AzureDevOpsProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        let path = url.path();

        let parsed = if url.hint() == GitUrlParseHint::Httplike {
            Self::parse_http_path(path)
        } else {
            Self::parse_ssh_path(path)
        };

        parsed.map(|(_, provider)| provider)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, CopyGetters, CloneGetters)]
pub struct GitLabProvider<'a> {
    #[getset(get_copy = "pub")]
    pub user: &'a str,
    #[getset(get_clone = "pub")]
    pub subgroup: Option<Vec<&'a str>>,
    #[getset(get_copy = "pub")]
    pub repo: &'a str,
}
impl<'a> GitLabProvider<'a> {
    fn parse_path(input: &str) -> Result<(&str, GitLabProvider), GitUrlParseError> {
        let parse_result = || -> IResult<&str, GitLabProvider> {
            // Optional leading slash
            let (input, _) = opt(tag("/")).parse(input)?;

            // Remove .git extension if present
            let input = input.trim_end_matches(".git");

            // Split the path
            let parts: Vec<&str> = input.split('/').filter(|s| !s.is_empty()).collect();

            // Ensure we have at least 2 parts (owner and repo)
            if parts.len() < 2 {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fail,
                )));
            }

            // Last part is the repo
            let repo = parts[parts.len() - 1];

            // Everything before the last part is the owner/subgroups
            let (user, subgroup) = if parts.len() > 2 {
                (parts[0], Some(parts[1..parts.len() - 1].to_vec()))
            } else {
                (parts[0], None)
            };

            Ok((
                input,
                GitLabProvider {
                    user,
                    subgroup,
                    repo,
                },
            ))
        };

        parse_result().map_err(|e| match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                GitUrlParseError::NomParseError(err.to_string())
            }
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedFormat,
        })
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GitLabProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}
