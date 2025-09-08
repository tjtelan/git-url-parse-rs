use crate::types::GitUrlParseHint;
use crate::{GitUrl, GitUrlParseError};

use getset::{CloneGetters, CopyGetters};
use nom::Parser;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::opt;
use nom::sequence::{preceded, separated_pair, terminated};

pub trait GitProvider<T, E>: Clone + std::fmt::Debug {
    fn from_git_url(url: &T) -> Result<Self, E>;
}

#[derive(Debug, PartialEq, Eq, Clone, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct GenericProvider<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl<'a> GenericProvider<'a> {
    fn parse_path(input: &str) -> Result<(&str, GenericProvider), GitUrlParseError> {
        let (input, _) = opt(tag("/")).parse(input)?;
        let (input, (user, repo)) =
            separated_pair(is_not("/"), tag("/"), take_until(".git")).parse(input)?;
        Ok((input, GenericProvider { owner: user, repo }))
    }

    pub fn fullname(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GenericProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        if url.hint() == GitUrlParseHint::Filelike {
            return Err(GitUrlParseError::ProviderUnsupported);
        }

        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct AzureDevOpsProvider<'a> {
    pub org: &'a str,
    pub project: &'a str,
    pub repo: &'a str,
}

impl<'a> AzureDevOpsProvider<'a> {
    fn parse_http_path(input: &str) -> Result<(&str, AzureDevOpsProvider), GitUrlParseError> {
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
    }

    fn parse_ssh_path(input: &str) -> Result<(&str, AzureDevOpsProvider), GitUrlParseError> {
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
    pub owner: &'a str,
    #[getset(get_clone = "pub")]
    pub subgroup: Option<Vec<&'a str>>,
    #[getset(get_copy = "pub")]
    pub repo: &'a str,
}

impl<'a> GitLabProvider<'a> {
    fn parse_path(input: &str) -> Result<(&str, GitLabProvider), GitUrlParseError> {
        // Optional leading slash
        let (input, _) = opt(tag("/")).parse(input)?;

        // Remove .git extension if present
        let input = input.trim_end_matches(".git");

        // Split the path
        let parts: Vec<&str> = input.split('/').filter(|s| !s.is_empty()).collect();

        // Ensure we have at least 2 parts (owner and repo)
        if parts.len() < 2 {
            return Err(GitUrlParseError::ProviderParseFail(
                "Path needs at least 2 parts: ex. \'/owner/repo\'".into(),
            ));
        }

        // Last part is the repo
        let repo = parts[parts.len() - 1];

        // Everything before the last part is the owner/subgroups
        let (owner, subgroup) = if parts.len() > 2 {
            (parts[0], Some(parts[1..parts.len() - 1].to_vec()))
        } else {
            (parts[0], None)
        };

        Ok((
            input,
            GitLabProvider {
                owner,
                subgroup,
                repo,
            },
        ))
    }
}

impl<'a> GitProvider<GitUrl<'a>, GitUrlParseError> for GitLabProvider<'a> {
    fn from_git_url(url: &GitUrl<'a>) -> Result<Self, GitUrlParseError> {
        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}
