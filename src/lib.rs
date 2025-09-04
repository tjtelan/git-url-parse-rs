pub mod types;

// Re-exports
pub use types::{
    GenericProvider, GitProvider, GitUrl, GitUrlBuilder, GitUrlBuilderError, GitUrlParseError,
};

#[cfg(feature = "tracing")]
use tracing::debug;
