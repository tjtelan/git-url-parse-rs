use super::GitProvider;
use crate::types::GitUrlParseHint;
use crate::{GitUrl, GitUrlParseError};

use getset::Getters;
use nom::Parser;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::opt;
use nom::sequence::{preceded, separated_pair, terminated};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "url")]
use url::Url;

/// Azure DevOps repository provider
/// ## Supported URL Formats
///
/// - `https://dev.azure.com/org/project/_git/repo`
/// - `git@ssh.dev.azure.com:v3/org/project/repo`
///
/// Example:
///
/// ```
/// use git_url_parse::{GitUrl, GitUrlParseError};
/// use git_url_parse::types::provider::AzureDevOpsProvider;
///
/// let test_url = "https://CompanyName@dev.azure.com/CompanyName/ProjectName/_git/RepoName";
/// let parsed = GitUrl::parse(test_url).expect("URL parse failed");
///
/// let provider_info: AzureDevOpsProvider = parsed.provider_info().unwrap();
///
/// assert_eq!(provider_info.org(), "CompanyName");
/// assert_eq!(provider_info.project(), "ProjectName");
/// assert_eq!(provider_info.repo(), "RepoName");
/// assert_eq!(provider_info.fullname(), "CompanyName/ProjectName/RepoName");
/// ```
///
#[derive(Debug, PartialEq, Eq, Clone, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get = "pub")]
pub struct AzureDevOpsProvider {
    /// Azure Devops organization name
    org: String,
    /// Azure Devops project name
    project: String,
    /// Azure Devops repo name
    repo: String,
}

impl AzureDevOpsProvider {
    /// Helper method to get the full name of a repo: `{org}/{project}/{repo}`
    pub fn fullname(&self) -> String {
        format!("{}/{}/{}", self.org, self.project, self.repo)
    }

    /// Parse the path of a http url for Azure Devops patterns
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

        Ok((
            input,
            AzureDevOpsProvider {
                org: org.to_string(),
                project: project.to_string(),
                repo: repo.to_string(),
            },
        ))
    }

    /// Parse the path of an ssh url for Azure Devops patterns
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

        Ok((
            input,
            AzureDevOpsProvider {
                org: org.to_string(),
                project: project.to_string(),
                repo: repo.to_string(),
            },
        ))
    }
}

impl GitProvider<GitUrl, GitUrlParseError> for AzureDevOpsProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        let path = url.path();

        let parsed = if url.hint() == GitUrlParseHint::Httplike {
            Self::parse_http_path(path)
        } else {
            Self::parse_ssh_path(path)
        };

        parsed.map(|(_, provider)| provider)
    }
}

#[cfg(feature = "url")]
impl GitProvider<Url, GitUrlParseError> for AzureDevOpsProvider {
    fn from_git_url(url: &Url) -> Result<Self, GitUrlParseError> {
        let path = url.path();

        let parsed = if url.scheme().contains("http") {
            Self::parse_http_path(path)
        } else {
            Self::parse_ssh_path(path)
        };

        parsed.map(|(_, provider)| provider)
    }
}
