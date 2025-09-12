#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(rustdoc::redundant_explicit_links)] // for cargo-rdme

//! # Git Url Parse
//!
//! Parses  url used by git (e.g. `git clone <url>`)
//!
//! ## Features
//!
//! - 🔍 Parses `git clone` compatible urls into [`GitUrl`](crate::types::GitUrl)
//!   - Supports multiple Git URL schemes (SSH, HTTP, HTTPS, File)
//!   - Inspired by [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986) with adaptations to support Git urls
//!
//! - 🏗️ Host provider info extraction
//!   - Easy to implement trait [`GitProvider`](crate::types::provider::GitProvider) for custom provider parsing
//!   - Built-in support for multiple Git hosting providers
//!       * [Generic](crate::types::provider::GenericProvider) (`git@host:owner/repo.git` style urls)
//!       * [GitLab](crate::types::provider::GitLabProvider)
//!       * [Azure DevOps](crate::types::provider::AzureDevOpsProvider)
//!
//! ## Quick Example
//!
//! ```rust
//! use git_url_parse::{GitUrl, GitUrlParseError};
//! use git_url_parse::types::provider::GitProvider;
//! use git_url_parse::types::provider::GenericProvider;
//!
//! fn main() -> Result<(), git_url_parse::GitUrlParseError> {
//!     let http_url = GitUrl::parse("https://github.com/tjtelan/git-url-parse-rs.git")?;
//!     
//!     // Extract basic URL components
//!     assert_eq!(http_url.host(), Some("github.com"));
//!     assert_eq!(http_url.path(), "/tjtelan/git-url-parse-rs.git");
//!
//!     // Support ssh-based urls as well
//!     let ssh_url = GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git")?;
//!
//!     assert_eq!(ssh_url.scheme(), Some("ssh"));
//!     assert_eq!(ssh_url.host(), Some("github.com"));
//!     assert_eq!(ssh_url.path(), "tjtelan/git-url-parse-rs.git");
//!     
//!     // Extract provider-specific information
//!     // Built-in support for Github (Generic), Gitlab, Azure Devops style urls
//!     let provider : GenericProvider = ssh_url.provider_info()?;
//!     assert_eq!(provider.owner(), "tjtelan");
//!     assert_eq!(provider.repo(), "git-url-parse-rs");
//!
//!     // Implement your own provider
//!     #[derive(Debug, Clone, PartialEq, Eq)]
//!     struct CustomProvider;
//!     
//!     impl GitProvider<GitUrl<'_>, GitUrlParseError> for CustomProvider {
//!         fn from_git_url(_url: &GitUrl) -> Result<Self, GitUrlParseError> {
//!             // Your custom provider parsing here
//!             Ok(Self)
//!         }
//!     }
//!
//!     let custom_provider: CustomProvider = ssh_url.provider_info()?;
//!     let expected = CustomProvider;
//!     assert_eq!(custom_provider, expected);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Limitations
//!
//!  Intended only for git repo urls. Url spec [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986) is not fully implemented.
//!
//! - No support for:
//!   - Query parameters
//!   - Fragment identifiers
//!   - Percent-encoding
//!   - Complex IP address formats
//!
//! ## Install
//!
//! ```shell
//! cargo add git-url-parse
//! ```
//!
//! ### Cargo Features
//!
//! #### `log`
//! Enable for internal `debug!` output from [log](https://docs.rs/log/latest)
//! #### `serde`
//! Enable for [serde](https://docs.rs/serde/latest/) `Serialize`/`Deserialize` on [`GitUrl`](crate::types::GitUrl)
//! #### `url`
//! (**enabled by default**)
//!
//! Uses [url](https://docs.rs/url/latest/) during parsing for full url validation
//!

pub mod types;

/// Re-exports
pub use types::{GitUrl, GitUrlParseError};
