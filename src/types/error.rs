use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitUrlParseError {
    #[error("Nom parsing error: {0}")]
    NomParseError(String),

    #[error("Git Url not in expected format")]
    UnexpectedFormat,

    #[error("Git Url not supported by provider")]
    ProviderUnsupported,

    #[error("Found null bytes within input url before parsing")]
    FoundNullBytes,
}
