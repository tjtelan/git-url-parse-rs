# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/tjtelan/git-url-parse-rs/compare/v0.5.0...v0.5.1) - 2025-09-13

### Other

- Update release-plz.yml ([#65](https://github.com/tjtelan/git-url-parse-rs/pull/65))
- Remove debug print ([#63](https://github.com/tjtelan/git-url-parse-rs/pull/63))

## [0.5.0](https://github.com/tjtelan/git-url-parse-rs/compare/v0.4.6...v0.5.0) - 2025-09-13

### Added

- Reimplement `GitUrl` with `nom` ([#61](https://github.com/tjtelan/git-url-parse-rs/pull/61))

## [0.4.6](https://github.com/tjtelan/git-url-parse-rs/compare/v0.4.5...v0.4.6) - 2025-09-13

### Fixed

- prevent panic when parsing a URL with no path ([#55](https://github.com/tjtelan/git-url-parse-rs/pull/55))

### Other

- Add release-plz to CI
- Enable default features for the url crate ([#54](https://github.com/tjtelan/git-url-parse-rs/pull/54))
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.5](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.5) - 2024-09-06

### CI

- Update changelog

### Changed

- Update MSRV badge
- Update README badges

### Fixed

- Add test for #51

### Other

- Reduce required dependencies

## [0.4.4](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.4) - 2022-11-05

### Fixed

- Handle case where parse fails on invalid port ([#50](https://github.com/tjtelan/git-url-parse-rs/issues/50))

## [0.4.3](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.3) - 2022-10-11

### Added

- Add short git URL notation support ([#28](https://github.com/tjtelan/git-url-parse-rs/issues/28))
- Add MSRV badge ([#36](https://github.com/tjtelan/git-url-parse-rs/issues/36))
- Add personal access token to checkout ([#41](https://github.com/tjtelan/git-url-parse-rs/issues/41))
- Add pre-merge generated updates ([#42](https://github.com/tjtelan/git-url-parse-rs/issues/42))
- Add support for wasm32-unknown-unknown compilation target ([#44](https://github.com/tjtelan/git-url-parse-rs/issues/44))

### Fixed

- Update CHANGELOG from Github Actions
- Make changelog update fixup to PR commit ([#33](https://github.com/tjtelan/git-url-parse-rs/issues/33))
- Fix post pr workflow ([#37](https://github.com/tjtelan/git-url-parse-rs/issues/37))
- Post PR: Include all PR files with changelog commit with `--all` ([#38](https://github.com/tjtelan/git-url-parse-rs/issues/38))
- Add dependency for update job completion before Bors merges ([#43](https://github.com/tjtelan/git-url-parse-rs/issues/43))
- Fix post merge lint ([#45](https://github.com/tjtelan/git-url-parse-rs/issues/45))

### Other

- Troubleshoot CI commit fail ([#39](https://github.com/tjtelan/git-url-parse-rs/issues/39))
- Troubleshoot Post PR ([#40](https://github.com/tjtelan/git-url-parse-rs/issues/40))
- Work out ci string parsing ([#48](https://github.com/tjtelan/git-url-parse-rs/issues/48))

## [0.4.2](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.2) - 2022-05-30

### Added

- Support Gerrit source code ([#24](https://github.com/tjtelan/git-url-parse-rs/issues/24))

### CI

- Replace log crate with tracing ([#25](https://github.com/tjtelan/git-url-parse-rs/issues/25))

## [0.4.1](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.1) - 2022-05-26

### Fixed

- Fix a panic case ([#21](https://github.com/tjtelan/git-url-parse-rs/issues/21))

### Other

- Ci tune ([#18](https://github.com/tjtelan/git-url-parse-rs/issues/18))

### Removed

- Update dependencies and readme ([#23](https://github.com/tjtelan/git-url-parse-rs/issues/23))

## [0.4.0](https://github.com/tjtelan/git-url-parse-rs/tree/v0.4.0) - 2021-11-14

### Added

- Adding release dates in changelog
- Rename workflow + add workflow_dispatch to ci

## [0.3.1](https://github.com/tjtelan/git-url-parse-rs/tree/v0.3.1) - 2021-01-27

### CI

- Updating Changelog to prepare for v0.3.1

### Other

- Loosens dependency restrictions ([#12](https://github.com/tjtelan/git-url-parse-rs/issues/12)) ([#13](https://github.com/tjtelan/git-url-parse-rs/issues/13))

## [0.3.0](https://github.com/tjtelan/git-url-parse-rs/tree/v0.3.0) - 2020-10-02

### Added

- Adding schemas

## [0.2.0](https://github.com/tjtelan/git-url-parse-rs/tree/v0.2.0) - 2020-05-13

### Added

- Adding build + test
- Adding badges to README.md

### Other

- Making enums and structs Clone

## [0.1.0](https://github.com/tjtelan/git-url-parse-rs/tree/v0.1.0) - 2020-02-05

### Added

- Adding docs url to Cargo.toml
- Adding support for Azure Devops repos

### Other

- Updating Cargo.toml for packaging

## [0.0.1](https://github.com/tjtelan/git-url-parse-rs/tree/v0.0.1) - 2020-01-22

### Other

- Initial commit
- Initial code commit

<!-- generated by git-cliff -->
