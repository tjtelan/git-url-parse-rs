//! # Git URL Providers
//!
//! Provides extraction of Git host service info from `GitUrl`s.
//!
//! ## Supported Providers
//!
//! - [Generic Git repositories](crate::types::provider::GenericProvider)
//! - [Azure DevOps](crate::types::provider::AzureDevOpsProvider)
//! - [GitLab](crate::types::provider::GitLabProvider)
//! - Custom (via [`GitProvider`] trait)

use crate::types::GitUrlParseHint;
use crate::{GitUrl, GitUrlParseError};

use getset::{CloneGetters, CopyGetters};
use nom::Parser;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::opt;
use nom::sequence::{preceded, separated_pair, terminated};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Secondary parser called by [`GitUrl::provider_info()`] to extract Git host provider info from url
///
/// ```
/// // Custom trait example
///
/// use git_url_parse::{GitUrl, GitUrlParseError};
/// use git_url_parse::types::provider::GitProvider;
///
///  #[derive(Debug, Clone, PartialEq, Eq)]
///  struct MyCustomProvider;
///
///  impl GitProvider<GitUrl<'_>, GitUrlParseError> for MyCustomProvider {
///      fn from_git_url(_url: &GitUrl) -> Result<Self, GitUrlParseError> {
///          // Do your custom parsing here with your GitUrl
///          Ok(Self)
///      }
///  }
///
///  let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
///  let parsed = GitUrl::parse(test_url).expect("URL parse failed");
///
///  // Provide your custom type to `GitUrl::provider_info()`
///  let provider_info: MyCustomProvider = parsed.provider_info().unwrap();
///  let expected = MyCustomProvider;
///  assert_eq!(provider_info, expected)
/// ```
pub trait GitProvider<T, E>: Clone + std::fmt::Debug {
    /// Trait method called by `GitUrl::provider_info()`
    ///
    /// Logic for extracting service level information from a `GitUrl`
    fn from_git_url(url: &T) -> Result<Self, E>;
}

/// Represents a generic Git repository provider
///
/// ## Typical Use Cases
///
/// - Common service hosting with `owner/repo` patterns (e.g. GitHub, Bitbucket)
/// - Self-hosted repositories (e.g. Codeberg, Gitea)
/// 
/// Example:
///
/// ```
/// use git_url_parse::{GitUrl, GitUrlParseError};
/// use git_url_parse::types::provider::GenericProvider;
/// 
/// let test_url = "git@github.com:tjtelan/git-url-parse-rs.git";
/// let parsed = GitUrl::parse(test_url).expect("URL parse failed");
/// 
/// let provider_info: GenericProvider = parsed.provider_info().unwrap();
/// 
/// assert_eq!(provider_info.owner(), "tjtelan");
/// assert_eq!(provider_info.repo(), "git-url-parse-rs");
/// assert_eq!(provider_info.fullname(), "tjtelan/git-url-parse-rs");
/// ```
/// 
#[derive(Debug, PartialEq, Eq, Clone, CopyGetters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get_copy = "pub")]
pub struct GenericProvider<'a> {
    /// Repo owner
    owner: &'a str,
    /// Repo name
    repo: &'a str,
}

impl<'a> GenericProvider<'a> {
    /// Parse the most common form of git url by offered by git providers
    fn parse_path(input: &str) -> Result<(&str, GenericProvider), GitUrlParseError> {
        let (input, _) = opt(tag("/")).parse(input)?;
        let (input, (user, repo)) =
            separated_pair(is_not("/"), tag("/"), take_until(".git")).parse(input)?;
        Ok((input, GenericProvider { owner: user, repo }))
    }

    /// Helper method to get the full name of a repo: `{owner}/{repo}`
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
#[derive(Debug, PartialEq, Eq, Clone, CopyGetters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get_copy = "pub")]
pub struct AzureDevOpsProvider<'a> {
    /// Azure Devops organization name
    org: &'a str,
    /// Azure Devops project name
    project: &'a str,
    /// Azure Devops repo name
    repo: &'a str,
}

impl<'a> AzureDevOpsProvider<'a> {
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

        Ok((input, AzureDevOpsProvider { org, project, repo }))
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

/// ## GitLab repository provider
///
/// ## Supported URL Formats
///
/// - `https://gitlab.com/owner/repo.git`
/// - `https://gitlab.com/owner/subgroup1/subgroup2/repo.git`
/// - `git@gitlab.com:owner/repo.git`
/// - `git@gitlab.com:owner/subgroup1/subgroup2/repo.git`
///
/// ## Examples
///
/// ```
/// use git_url_parse::GitUrl;
/// use git_url_parse::types::provider::GitLabProvider;
///
/// fn main() -> Result<(), git_url_parse::GitUrlParseError> {
///     // Top-level repository
///     let url1 = GitUrl::parse("https://gitlab.com/gitlab-org/gitlab.git")?;
///     let provider1 : GitLabProvider = url1.provider_info()?;
///     assert_eq!(provider1.owner(), "gitlab-org");
///     assert_eq!(provider1.repo(), "gitlab");
///     assert_eq!(provider1.subgroup(), None);
///     assert_eq!(provider1.fullname(), "gitlab-org/gitlab");
///
///     // Repository with subgroups
///     let url2 = GitUrl::parse("https://gitlab.com/owner/group1/group2/project.git")?;
///     let provider2 : GitLabProvider = url2.provider_info()?;
///     assert_eq!(provider2.owner(), "owner");
///     assert_eq!(provider2.repo(), "project");
///     assert_eq!(provider2.subgroup(), Some(vec!["group1", "group2"]));
///     assert_eq!(provider2.fullname(), "owner/group1/group2/project");
///
///     Ok(())
/// }
/// ```
/// 
#[derive(Clone, Debug, PartialEq, Eq, Default, CopyGetters, CloneGetters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GitLabProvider<'a> {
    /// Repo owner
    #[getset(get_copy = "pub")]
    owner: &'a str,
    /// Gitlab subgroups
    #[getset(get_clone = "pub")]
    subgroup: Option<Vec<&'a str>>,
    /// Repo name
    #[getset(get_copy = "pub")]
    repo: &'a str,
}

impl<'a> GitLabProvider<'a> {
    /// Helper method to get the full name of a repo: `{owner}/{repo}` or `{owner}/{subgroups}/{repo}`
    pub fn fullname(&self) -> String {
        if let Some(subgroup) = self.subgroup() {
            let subgroup_str = subgroup.join("/");

            format!("{}/{subgroup_str}/{}", self.owner, self.repo)
        } else {
            format!("{}/{}", self.owner, self.repo)
        }
    }

    /// Parse the path of url for GitLab patterns
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
