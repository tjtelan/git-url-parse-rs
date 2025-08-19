mod provider;
mod types;
pub use types::{GitUrl, GitUrlBuilder, GitUrlBuilderError, GitUrlParseError, Scheme};

#[cfg(feature = "tracing")]
use tracing::debug;
