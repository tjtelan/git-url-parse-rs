use std::str::FromStr;

pub mod types;
pub use types::scheme::Scheme;
pub use types::giturl::GitUrl;

use color_eyre::eyre::{eyre, WrapErr};
pub use color_eyre::Result;
use regex::Regex;
use tracing::debug;
use url::Url;

/// `normalize_ssh_url` takes in an ssh url that separates the login info
/// from the path into with a `:` and replaces it with `/`.
///
/// Prepends `ssh://` to url
///
/// Supports absolute and relative paths
pub(crate) fn normalize_ssh_url(url: &str) -> Result<Url> {
    let u = url.split(':').collect::<Vec<&str>>();

    match u.len() {
        2 => {
            debug!("Normalizing ssh url: {:?}", u);
            normalize_url(&format!("ssh://{}/{}", u[0], u[1]))
        }
        3 => {
            debug!("Normalizing ssh url with ports: {:?}", u);
            normalize_url(&format!("ssh://{}:{}/{}", u[0], u[1], u[2]))
        }
        _default => Err(eyre!("SSH normalization pattern not covered for: {:?}", u)),
    }
}

/// `normalize_file_path` takes in a filepath and uses `Url::from_file_path()` to parse
///
/// Prepends `file://` to url
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi"))]
pub(crate) fn normalize_file_path(filepath: &str) -> Result<Url> {
    let fp = Url::from_file_path(filepath);

    match fp {
        Ok(path) => Ok(path),
        Err(_e) => Ok(normalize_url(&format!("file://{}", filepath))
            .with_context(|| "file:// normalization failed".to_string())?),
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn normalize_file_path(_filepath: &str) -> Result<Url> {
    unreachable!()
}

/// `normalize_url` takes in url as `&str` and takes an opinionated approach to identify
/// `ssh://` or `file://` urls that require more information to be added so that
/// they can be parsed more effectively by `url::Url::parse()`
pub fn normalize_url(url: &str) -> Result<Url> {
    debug!("Processing: {:?}", &url);

    // Error if there are null bytes within the url
    // https://github.com/tjtelan/git-url-parse-rs/issues/16
    if url.contains('\0') {
        return Err(eyre!("Found null bytes within input url before parsing"));
    }

    // We're going to remove any trailing slash before running through Url::parse
    let trim_url = url.trim_end_matches('/');

    // normalize short git url notation: git:host/path
    let url_to_parse = if Regex::new(r"^git:[^/]")
        .with_context(|| "Failed to build short git url regex for testing against url".to_string())?
        .is_match(trim_url)
    {
        trim_url.replace("git:", "git://")
    } else {
        trim_url.to_string()
    };

    let url_parse = Url::parse(&url_to_parse);

    Ok(match url_parse {
        Ok(u) => {
            match Scheme::from_str(u.scheme()) {
                Ok(_p) => u,
                Err(_e) => {
                    // Catch case when an ssh url is given w/o a user
                    debug!("Scheme parse fail. Assuming a userless ssh url");
                    normalize_ssh_url(trim_url).with_context(|| {
                        "No url scheme was found, then failed to normalize as ssh url.".to_string()
                    })?
                }
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // If we're here, we're only looking for Scheme::Ssh or Scheme::File

            // Assuming we have found Scheme::Ssh if we can find an "@" before ":"
            // Otherwise we have Scheme::File
            let re = Regex::new(r"^\S+(@)\S+(:).*$").with_context(|| {
                "Failed to build ssh git url regex for testing against url".to_string()
            })?;

            match re.is_match(trim_url) {
                true => {
                    debug!("Scheme::SSH match for normalization");
                    normalize_ssh_url(trim_url)
                        .with_context(|| "Failed to normalize as ssh url".to_string())?
                }
                false => {
                    debug!("Scheme::File match for normalization");
                    normalize_file_path(trim_url)
                        .with_context(|| "Failed to normalize as file url".to_string())?
                }
            }
        }
        Err(err) => {
            return Err(eyre!("url parsing failed: {:?}", err));
        }
    })
}
