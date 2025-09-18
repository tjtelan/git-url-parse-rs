use super::GitProvider;
use crate::types::GitUrlParseHint;
use crate::{GitUrl, GitUrlParseError};

use getset::Getters;
use nom::Parser;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::opt;
use nom::sequence::separated_pair;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "url")]
use url::Url;

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
#[derive(Debug, PartialEq, Eq, Clone, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get = "pub")]
pub struct GenericProvider {
    /// Repo owner
    owner: String,
    /// Repo name
    repo: String,
}

impl GenericProvider {
    /// Parse the most common form of git url by offered by git providers
    fn parse_path(input: &str) -> Result<(&str, GenericProvider), GitUrlParseError> {
        let (input, _) = opt(tag("/")).parse(input)?;
        let (input, (user, repo)) = if input.ends_with(".git") {
            separated_pair(is_not("/"), tag("/"), take_until(".git")).parse(input)?
        } else {
            separated_pair(is_not("/"), tag("/"), is_not("/")).parse(input)?
        };
        Ok((
            input,
            GenericProvider {
                owner: user.to_string(),
                repo: repo.to_string(),
            },
        ))
    }

    /// Helper method to get the full name of a repo: `{owner}/{repo}`
    pub fn fullname(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl GitProvider<GitUrl, GitUrlParseError> for GenericProvider {
    fn from_git_url(url: &GitUrl) -> Result<Self, GitUrlParseError> {
        if url.hint() == GitUrlParseHint::Filelike {
            return Err(GitUrlParseError::ProviderUnsupported);
        }

        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}

#[cfg(feature = "url")]
impl GitProvider<Url, GitUrlParseError> for GenericProvider {
    fn from_git_url(url: &Url) -> Result<Self, GitUrlParseError> {
        if url.scheme() == "file" {
            return Err(GitUrlParseError::ProviderUnsupported);
        }

        let path = url.path();
        Self::parse_path(path).map(|(_, provider)| provider)
    }
}
