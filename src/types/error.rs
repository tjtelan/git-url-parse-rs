//! # GitUrl error handling
//!
//! Error struct to use as Err for parsing Git urls

use thiserror::Error;

/// Internal error type for `GitUrl` for parsing errors
#[derive(Error, Debug, PartialEq, Eq)]
pub enum GitUrlParseError {
    #[cfg(feature = "url")]
    /// Error originating from from `url` crate during validation
    #[error("Error from Url crate: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// Parsing error converted from `nom` crate
    #[error("Nom crate parsing error: {0}")]
    NomParseError(String),

    /// Git url must contain a non-empty path
    #[error("Git Url must have a path")]
    InvalidPathEmpty,

    /// Invalid port number detected
    #[error("Invalid port number")]
    InvalidPortNumber,

    /// Password are only supported in HTTP-like url
    #[error("Password only supported by httplike urls")]
    InvalidPasswordUnsupported,

    /// File-like url must follow filesystem path patterns
    #[error("Filelike urls expect only scheme and/or path")]
    InvalidFilePattern,

    /// `GitUrl`not supported by the [`GitProvider`](crate::types::provider::GitProvider)
    #[error("GitUrl not supported by provider")]
    ProviderUnsupported,

    /// Detected null bytes in the input url
    #[error("Found null bytes within input url before parsing")]
    FoundNullBytes,

    /// Failed to extract provider-specific info from url
    #[error("Provider info parse failed: {0}")]
    ProviderParseFail(String),

    /// Catch-all error for unexpected failures during parsing
    #[error("Unexpected error occurred during parsing")]
    UnexpectedError,
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for GitUrlParseError {
    fn from(err: nom::Err<(&'a str, nom::error::ErrorKind)>) -> Self {
        match err {
            nom::Err::Error((input, kind)) => {
                GitUrlParseError::NomParseError(format!("Parse error at: {input}, kind: {kind:?}",))
            }
            nom::Err::Failure((input, kind)) => {
                GitUrlParseError::NomParseError(format!("Parse error at: {input}, kind: {kind:?}",))
            }
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedError,
        }
    }
}
