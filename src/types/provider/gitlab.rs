use super::GitProvider;
use crate::{GitUrl, GitUrlParseError};

use getset::{CloneGetters, Getters};
use nom::Parser;
use nom::bytes::complete::tag;
use nom::combinator::opt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "url")]
use url::Url;

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
#[derive(Clone, Debug, PartialEq, Eq, Default, Getters, CloneGetters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GitLabProvider {
    /// Repo owner
    #[getset(get = "pub")]
    owner: String,
    /// Gitlab subgroups
    //#[getset(get_clone = "pub")]
    subgroup: Option<Vec<String>>,
    /// Repo name
    #[getset(get = "pub")]
    repo: String,
}

impl GitLabProvider {
    /// Repo owner
    /// Gitlab subgroups
    pub fn subgroup(&self) -> Option<Vec<&str>> {
        if let Some(s) = &self.subgroup {
            let subgroup_vec: Vec<&str> = s.iter().map(|s| s.as_str()).collect();
            Some(subgroup_vec)
        } else {
            None
        }
    }

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
        let repo = parts[parts.len() - 1].to_string();

        // Everything before the last part is the owner/subgroups
        let (owner, subgroup) = if parts.len() > 2 {
            let subgroup: Vec<String> = parts[1..(parts.len() - 1)]
                .iter()
                .copied()
                .map(|s| s.to_string())
                .collect();

            (parts[0].to_string(), Some(subgroup))
        } else {
            (parts[0].to_string(), None)
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

impl GitProvider<GitUrl, GitUrlParseError> for GitLabProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}

#[cfg(feature = "url")]
impl GitProvider<Url, GitUrlParseError> for GitLabProvider {
    fn from_git_url(url: &Url) -> Result<Self, GitUrlParseError> {
        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}
