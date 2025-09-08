use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GitUrlParseError {
    #[error("Error from Url crate: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Nom crate parsing error: {0}")]
    NomParseError(String),

    #[error("Git Url must have a path")]
    InvalidPathEmpty,

    #[error("Invalid port number")]
    InvalidPortNumber,

    #[error("Tokens only supported by httplike urls")]
    InvalidTokenUnsupported,

    #[error("Filelike urls expect only scheme and/or path")]
    InvalidFilePattern,

    #[error("Git Url not supported by provider")]
    ProviderUnsupported,

    #[error("Found null bytes within input url before parsing")]
    FoundNullBytes,

    #[error("Provider info parse failed: {0}")]
    ProviderParseFail(String),

    #[error("Unexpected error occurred during parsing")]
    UnexpectedError,
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for GitUrlParseError {
    fn from(err: nom::Err<(&'a str, nom::error::ErrorKind)>) -> Self {
        match err {
            nom::Err::Error((input, kind)) => GitUrlParseError::NomParseError(format!(
                "Parse error at: {}, kind: {:?}",
                input, kind
            )),
            nom::Err::Failure((input, kind)) => GitUrlParseError::NomParseError(format!(
                "Parse failure at: {}, kind: {:?}",
                input, kind
            )),
            nom::Err::Incomplete(_) => GitUrlParseError::UnexpectedError,
        }
    }
}
