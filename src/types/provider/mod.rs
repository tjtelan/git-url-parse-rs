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

mod azure_devops;
mod generic;
mod gitlab;

pub use azure_devops::AzureDevOpsProvider;
pub use generic::GenericProvider;
pub use gitlab::GitLabProvider;

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
///  impl GitProvider<GitUrl, GitUrlParseError> for MyCustomProvider {
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
