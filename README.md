# git-url-parse

[![Crates.io](https://img.shields.io/crates/v/git-url-parse)](https://crates.io/crates/git-url-parse)
[![docs.rs](https://docs.rs/git-url-parse/badge.svg)](https://docs.rs/git-url-parse/)
[![licence](https://img.shields.io/github/license/tjtelan/git-url-parse-rs)](LICENSE)
![Github actions build status](https://github.com/tjtelan/git-url-parse-rs/workflows/git-url-parse/badge.svg)

Supports common protocols as specified by the [Pro Git book](https://git-scm.com/book/en/v2)

See: [4.1 Git on the Server - The Protocols](https://git-scm.com/book/en/v2/Git-on-the-Server-The-Protocols)

Supports parsing SSH/HTTPS repo urls for:
* Github
* Bitbucket
* Azure Devops

See `tests/parse.rs` for expected output for a variety of inputs.

---

URLs that use the `ssh://` protocol (implicitly or explicitly) undergo a small normalization process in order to be parsed.

Internally uses `Url::parse()` from the [Url](https://crates.io/crates/url) crate after normalization.

## Example usage
```rust
use git_url_parse::GitUrl;

fn main() {
    println!("SSH: {:?}", GitUrl::parse("git@github.com:tjtelan/git-url-parse-rs.git"));
    println!("HTTPS: {:?}", GitUrl::parse("https://github.com/tjtelan/git-url-parse-rs"));
}
```

## Example Output
```bash
SSH: Ok(GitUrl { href: "git@github.com:tjtelan/git-url-parse-rs.git", host: Some("github.com"), name: "git-url-parse-rs", owner: Some("tjtelan"), organization: None, fullname: "tjtelan/git-url-parse-rs", protocol: Ssh, user: Some("git"), token: None, port: None, path: "tjtelan/git-url-parse-rs.git", git_suffix: true })
HTTPS: Ok(GitUrl { href: "https://github.com/tjtelan/git-url-parse-rs", host: Some("github.com"), name: "git-url-parse-rs", owner: Some("tjtelan"), organization: None, fullname: "tjtelan/git-url-parse-rs", protocol: Https, user: None, token: None, port: None, path: "/tjtelan/git-url-parse-rs", git_suffix: false })
```