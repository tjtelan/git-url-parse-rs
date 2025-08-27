mod types;
pub use types::{
    GenericProvider, GitProvider, GitUrl, GitUrlBuilder, GitUrlBuilderError, GitUrlParseError,
    Scheme,
};

#[cfg(feature = "tracing")]
use tracing::debug;
