# git-url-parse

![Minimum Supported Rust Version](https://raw.githubusercontent.com/tjtelan/git-url-parse-rs/main/.github/assets/msrv-badge.svg)
[![Crates.io](https://img.shields.io/crates/v/git-url-parse)](https://crates.io/crates/git-url-parse)
[![Github actions CI status](https://github.com/tjtelan/git-url-parse-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tjtelan/git-url-parse-rs/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/git-url-parse/badge.svg)](https://docs.rs/git-url-parse/)
[![License](https://img.shields.io/github/license/tjtelan/git-url-parse-rs)](LICENSE)
![Maintenance](https://img.shields.io/maintenance/yes/2024)

Supports common protocols as specified by the [Pro Git book](https://git-scm.com/book/en/v2)

See: [4.1 Git on the Server - The Protocols](https://git-scm.com/book/en/v2/Git-on-the-Server-The-Protocols)

Supports parsing SSH/HTTPS repo urls for:
* Github
* Bitbucket
* Azure Devops

See [tests/parse.rs](tests/parse.rs) for expected output for a variety of inputs.

---

URLs that use the `ssh://` protocol (implicitly or explicitly) undergo a small normalization process in order to be parsed.

Internally uses `Url::parse()` from the [Url](https://crates.io/crates/url) crate after normalization.

## Examples

### Run example with debug output

```shell
$ RUST_LOG=git_url_parse cargo run --example multi
$ RUST_LOG=git_url_parse cargo run --example trim_auth 
```

### Simple usage and output

```bash
$ cargo run --example readme
```

```rust
use git_url_parse::GitUrl;

fn main() {
    println!("SSH: {:#?}", GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git"));
    println!("HTTPS: {:#?}", GitUrl::parse("https://github.com/tjtelan/git-url-parse-rs"));
}
```

### Example Output
```bash
SSH: Ok(
    GitUrl {
        host: Some(
            "github.com",
        ),
        name: "git-url-parse-rs",
        owner: Some(
            "tjtelan",
        ),
        organization: None,
        fullname: "tjtelan/git-url-parse-rs",
        scheme: Ssh,
        user: Some(
            "git",
        ),
        token: None,
        port: None,
        path: "tjtelan/git-url-parse-rs.git",
        git_suffix: true,
        scheme_prefix: false,
    },
)
HTTPS: Ok(
    GitUrl {
        host: Some(
            "github.com",
        ),
        name: "git-url-parse-rs",
        owner: Some(
            "tjtelan",
        ),
        organization: None,
        fullname: "tjtelan/git-url-parse-rs",
        scheme: Https,
        user: None,
        token: None,
        port: None,
        path: "/tjtelan/git-url-parse-rs",
        git_suffix: false,
        scheme_prefix: true,
    },
)
```