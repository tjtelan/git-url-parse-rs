# [0.3.0](https://github.com/tjtelan/git-url-parse-rs/compare/v0.2.0...v0.3.0)
- Add `CHANGELOG.md`
- Add new schemes `Ftp` and `Ftps`
- Update `GitUrl` format for `Scheme::File` for `GitUrl.host` and `GitUrl.path`
- Add more tests

# [0.2.0](https://github.com/tjtelan/git-url-parse-rs/compare/v0.1.1...v0.2.0)
- Updating `GitUrl` format 
- Add `trim_auth()`
- Add `impl Display` for `GitUrl`
- Rename enum `Protocol` to `Scheme`

# [0.1.1](https://github.com/tjtelan/git-url-parse-rs/compare/v0.1.0...v0.1.1)
- Add CI via Github Actions
- Add badges for docs, crates, build to `README`
- Update `README.md`
- Making enums and Structs `Clone`

# [0.1.0](https://github.com/tjtelan/git-url-parse-rs/compare/v0.0.1...v0.1.0)
- Add support for Azure DevOps
- Add `README.md`

# [0.0.1](https://github.com/tjtelan/git-url-parse-rs/commit/9255fc3f0516e6cfa60c651dd0436fa702b701b1)
- Pre-process urls before feeding to `url` crate
- Specialized normalization rules between ssh or file path urls