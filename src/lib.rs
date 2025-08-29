mod types;
pub use types::{
    GenericProvider, GitProvider, GitUrl, GitUrlBuilder, GitUrlBuilderError, GitUrlParseError,
    Scheme, AzureDevOpsProvider, GitLabProvider,
};

#[cfg(feature = "tracing")]
use tracing::debug;
