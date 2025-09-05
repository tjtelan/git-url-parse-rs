use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitUrlParseError {
    //#[error("Error from derive_builder")]
    //DeriveBuilderError(#[from] GitUrlOldBuilderError),

    //#[error("Error from Url crate: {0}")]
    //UrlParseError(#[from] url::ParseError),

    //#[error("No url scheme was found, then failed to normalize as ssh url.")]
    //SshUrlNormalizeFailedNoScheme,

    //#[error("No url scheme was found, then failed to normalize as ssh url after adding 'ssh://'")]
    //SshUrlNormalizeFailedSchemeAdded,

    //#[error("Failed to normalize as ssh url after adding 'ssh://'")]
    //SshUrlNormalizeFailedSchemeAddedWithPorts,

    //#[error("No url scheme was found, then failed to normalize as file url.")]
    //FileUrlNormalizeFailedNoScheme,

    //#[error("No url scheme was found, then failed to normalize as file url after adding 'file://'")]
    //FileUrlNormalizeFailedSchemeAdded,
    #[error("Git Url not in expected format")]
    UnexpectedFormat,

    // FIXME: Keep an eye on this error for removal
    #[error("Git Url for host using unexpected scheme")]
    UnexpectedScheme,

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
