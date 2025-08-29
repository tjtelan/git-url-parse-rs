mod types;
pub use types::{
    AzureDevOpsProvider, GenericProvider, GitLabProvider, GitProvider, GitUrl, GitUrlBuilder,
    GitUrlBuilderError, GitUrlParseError, Scheme,
};

#[cfg(feature = "tracing")]
use tracing::debug;
