mod types;
pub use types::{
    GenericProvider, GitUrl, GitUrlBuilder, GitUrlBuilderError, GitUrlParseError, Scheme,
};

#[cfg(feature = "tracing")]
use tracing::debug;
