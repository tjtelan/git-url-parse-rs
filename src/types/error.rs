use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitUrlParseError {
    #[error("Nom parsing error: {0}")]
    NomParseError(String),


    #[error("Git Url not in expected format")]
    UnexpectedFormat,

    // FIXME: Keep an eye on this error for removal
    #[error("Git Url for host using unexpected scheme")]
    UnexpectedScheme,

    #[error("Git Url not supported by provider")]
    ProviderUnsupported,

    //#[error("Scheme unsupported: {0}")]
    //UnsupportedScheme(String),
    //#[error("Host from Url cannot be str or does not exist")]
    //UnsupportedUrlHostFormat,
    //#[error("Git Url not in expected format for SSH")]
    //UnsupportedSshUrlFormat,
    //#[error("Normalized URL has no path")]
    //EmptyPath,
    #[error("Found null bytes within input url before parsing")]
    FoundNullBytes,

    // Maybe remove this. Handled by derive_builder
    #[error("Value expected for field: {0}")]
    UnexpectedEmptyValue(String),
}
