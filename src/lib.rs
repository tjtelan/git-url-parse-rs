pub mod types;

// Re-exports
pub use types::{GenericProvider, GitProvider, GitUrl, GitUrlParseError};

#[cfg(feature = "tracing")]
use tracing::debug;
