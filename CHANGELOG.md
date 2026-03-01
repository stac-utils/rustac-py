# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.8](https://github.com/stac-utils/rustac-py/compare/v0.9.7...v0.9.8) (2026-03-01)


### Bug Fixes

* duckdb extension grabs ([#245](https://github.com/stac-utils/rustac-py/issues/245)) ([643dead](https://github.com/stac-utils/rustac-py/commit/643dead8eb7bfa063a3491e07630150c5d9e5e1b))


### Dependencies

* cargo update ([#244](https://github.com/stac-utils/rustac-py/issues/244)) ([99f4a14](https://github.com/stac-utils/rustac-py/commit/99f4a149ce5e52c269aa5327176f3eefe5299d67))

## [0.9.7](https://github.com/stac-utils/rustac-py/compare/v0.9.6...v0.9.7) (2026-02-28)


### Features

* duckdb extensions via download ([#242](https://github.com/stac-utils/rustac-py/issues/242)) ([fa6937e](https://github.com/stac-utils/rustac-py/commit/fa6937e1c0145bbe31b79e3408edda1c1da732f6))


### Bug Fixes

* pre-release for duckdb versioning ([9c9022a](https://github.com/stac-utils/rustac-py/commit/9c9022a4ce4c1ed473235955b6a6d5e8b7fa3f91))
* release as ([859a206](https://github.com/stac-utils/rustac-py/commit/859a206de3f1aa715131b0b9d4b24129e15046ab))
* release-please manifest ([0e54028](https://github.com/stac-utils/rustac-py/commit/0e54028f33f587d11ffe0f60c5ed3146ed67aee6))

## [0.9.6](https://github.com/stac-utils/rustac-py/compare/v0.9.5...v0.9.6) (2026-02-12)


### Bug Fixes

* update windows paths ([#238](https://github.com/stac-utils/rustac-py/issues/238)) ([8f9a1b6](https://github.com/stac-utils/rustac-py/commit/8f9a1b63e5e07c14448695d8b3ae14a1a6e9e91b))


### Documentation

* add read_sync, write_sync, and search_sync ([#234](https://github.com/stac-utils/rustac-py/issues/234)) ([921e3f9](https://github.com/stac-utils/rustac-py/commit/921e3f93dc419f7cc4537250264f9469a3f15a3d))

## [0.9.5](https://github.com/stac-utils/rustac-py/compare/v0.9.4...v0.9.5) (2026-02-04)


### Features

* add read, write, and search *_sync ([#232](https://github.com/stac-utils/rustac-py/issues/232)) ([f0363d5](https://github.com/stac-utils/rustac-py/commit/f0363d51008760b6cb86a2f9958d79955ee26f48))

## [0.9.4](https://github.com/stac-utils/rustac-py/compare/v0.9.3...v0.9.4) (2026-02-02)


### Bug Fixes

* add max_items to duckdb client ([#229](https://github.com/stac-utils/rustac-py/issues/229)) ([4e90097](https://github.com/stac-utils/rustac-py/commit/4e900977b95b44ca57a408b631ae1a3364275448)), closes [#228](https://github.com/stac-utils/rustac-py/issues/228)


### Dependencies

* object_store goes with arrow ([#219](https://github.com/stac-utils/rustac-py/issues/219)) ([8f5c994](https://github.com/stac-utils/rustac-py/commit/8f5c9940d286365df12ba56b8777eba74f4ab87a))

## [0.9.3](https://github.com/stac-utils/rustac-py/compare/v0.9.2...v0.9.3) (2026-01-06)


### Features

* add collections to parquet metadata ([#208](https://github.com/stac-utils/rustac-py/issues/208)) ([617ecca](https://github.com/stac-utils/rustac-py/commit/617ecca449c187b1d2bda4e49294f82f48d439e1))
* normalize datetimes when searching ([#202](https://github.com/stac-utils/rustac-py/issues/202)) ([a1b5660](https://github.com/stac-utils/rustac-py/commit/a1b5660432886f6448cb15981be11fe4a5f02c32))


### Bug Fixes

* split cargo dependencies ([#195](https://github.com/stac-utils/rustac-py/issues/195)) ([14919bb](https://github.com/stac-utils/rustac-py/commit/14919bb981edeaffdd28b9580dc8e09bb9ff5196))


### Dependencies

* bump a few ([#190](https://github.com/stac-utils/rustac-py/issues/190)) ([f2c6816](https://github.com/stac-utils/rustac-py/commit/f2c6816649465c568b5313729b61bb1bfd42ee61))

## [0.10.0](https://github.com/stac-utils/rustac-py/compare/v0.9.2...v0.10.0) (2026-01-06)


### Features

* add collections to parquet metadata ([#208](https://github.com/stac-utils/rustac-py/issues/208)) ([617ecca](https://github.com/stac-utils/rustac-py/commit/617ecca449c187b1d2bda4e49294f82f48d439e1))
* normalize datetimes when searching ([#202](https://github.com/stac-utils/rustac-py/issues/202)) ([a1b5660](https://github.com/stac-utils/rustac-py/commit/a1b5660432886f6448cb15981be11fe4a5f02c32))


### Bug Fixes

* split cargo dependencies ([#195](https://github.com/stac-utils/rustac-py/issues/195)) ([14919bb](https://github.com/stac-utils/rustac-py/commit/14919bb981edeaffdd28b9580dc8e09bb9ff5196))


### Dependencies

* bump a few ([#190](https://github.com/stac-utils/rustac-py/issues/190)) ([f2c6816](https://github.com/stac-utils/rustac-py/commit/f2c6816649465c568b5313729b61bb1bfd42ee61))

## [Unreleased]

## [0.9.2]

### Fixed

- Circular import issue ([#186](https://github.com/stac-utils/rustac-py/pull/186))

## [0.9.1]

### Added

- Write **stac-geoparquet** to an object store ([#182](https://github.com/stac-utils/rustac-py/pull/182))

## [0.9.0]

### Added

- Chunked stac-geoparquet writing ([#178](https://github.com/stac-utils/rustac-py/pull/178))

### Removed

- STAC typed dicts ([#177](https://github.com/stac-utils/rustac-py/pull/177))

## [0.8.4] - 2025-10-22

### Fixed

- Deconstruct item collections when writing ndjson ([#167](https://github.com/stac-utils/rustac-py/pull/167))

## [0.8.3] - 2025-09-24

### Changed

- Don't set geo metadata for `proj:geometry` ([rustac #808](https://github.com/stac-utils/rustac/pull/808), [#161](https://github.com/stac-utils/rustac-py/pull/161))

## [0.8.2] - 2025-09-15

Bump **pyo3** version.

## [0.8.1] - 2025-06-16

### Added

- `type` field to geoparquet writes ([#136](https://github.com/stac-utils/rustac-py/pull/136), <https://github.com/stac-utils/rustac/pull/736>)
- `parquet_compression` argument to `write` and `search_to` ([#150](https://github.com/stac-utils/rustac-py/pull/150))
- `iter_search` ([#151](https://github.com/stac-utils/rustac-py/pull/151))
- `union_by_name` when searching **stac-geoparquet** ([#152](https://github.com/stac-utils/rustac-py/pull/152))

### Fixed

- Error instead of panic for cql ([#138](https://github.com/stac-utils/rustac-py/pull/138), <https://github.com/developmentseed/cql2-rs/pull/83>)

## [0.8.0] - 2025-05-13

### Added

- `rustac.store` ([#127](https://github.com/stac-utils/rustac-py/pull/127))
- More linux wheels ([#132](https://github.com/stac-utils/rustac-py/pull/132))

### Removed

- `options` from `read`, `write`, and `search_to` ([#127](https://github.com/stac-utils/rustac-py/pull/127), [#130](https://github.com/stac-utils/rustac-py/pull/130))

## [0.7.2] - 2025-05-05

### Fixed

- Search ([#122](https://github.com/stac-utils/rustac-py/pull/122))
- Reading `proj:geometry` (and other geometries) ([#125](https://github.com/stac-utils/rustac-py/pull/125))

## [0.7.1] - 2025-05-02

### Fixed

- `proj:geometry` ([#120](https://github.com/stac-utils/rustac-py/pull/120))

## [0.7.0] - 2025-04-29

### Added

- Source distribution to PyPI publish ([#92](https://github.com/stac-utils/rustac-py/pull/92))
- `rustac.sha` ([#99](https://github.com/stac-utils/rustac-py/pull/99))
- Typed dictionaries for STAC entities ([#101](https://github.com/stac-utils/rustac-py/pull/101))
- `rustac.collection_from_id_and_items` ([#109](https://github.com/stac-utils/rustac-py/pull/109))

### Fixed

- Deterministic asset ordering ([rustac #709](https://github.com/stac-utils/rustac/pull/709), [#93](https://github.com/stac-utils/rustac-py/pull/93))
- Normalize search output ([#102](https://github.com/stac-utils/rustac-py/pull/102))

### Removed

- Python 3.10 support ([#110](https://github.com/stac-utils/rustac-py/pull/110))

## [0.6.0] - 2025-04-18

> [!NOTE]
> This package was renamed from **stacrs** to **rustac**.

### Added

- Construct `stac_api::Search` (moved from `stac_api` crate) ([#81](https://github.com/stac-utils/rustac-py/pull/81))

### Fixed

- Swallow broken pipe errors ([#73](https://github.com/stac-utils/rustac-py/pull/73))
- Clean up docs ([#78](https://github.com/stac-utils/rustac-py/pull/78))

### Removed

- `migrate_href` ([#78](https://github.com/stac-utils/rustac-py/pull/78))

## [0.5.9] - 2025-03-03

### Added

- `walk` and `set_self_link` for `read` ([#69](https://github.com/stac-utils/rustac-py/pull/69))

## [0.5.8] - 2025-02-27

### Fixed

- Patch DuckDB ([#64](https://github.com/stac-utils/rustac-py/pull/64))

## [0.5.7] - 2025-02-26

### Changed

- Don't include libduckdb, but rather build bundled to save size ([#61](https://github.com/stac-utils/rustac-py/pull/61))

## [0.5.6] - 2025-02-26

### Added

- Search to an arrow table ([#54](https://github.com/stac-utils/rustac-py/pull/54))
- Create a item collection from an arrow table ([#57](https://github.com/stac-utils/rustac-py/pull/57))

### Changed

- Include **libduckdb** in wheels ([#52](https://github.com/stac-utils/rustac-py/pull/52))

## [0.5.5] - 2025-02-20

### Fixed

- Removed tracing subscriber to fix CLI ([#49](https://github.com/stac-utils/rustac-py/pull/49))

## [0.5.4] - 2025-02-19

### Added

- CLI ([#46](https://github.com/stac-utils/rustac-py/pull/46))
- Config args to DuckDB client ([#42](https://github.com/stac-utils/rustac-py/pull/42))

## [0.5.3] - 2025-02-07

### Changed

- Use only abi3 wheels ([#36](https://github.com/gadomski/rustac-py/pull/36))

> [!WARNING]
> All versions older than v0.5.3 were deleted from PyPI, but some tags still exist on this repo.
> See <https://github.com/gadomski/rustac-py/discussions/37> for more.

## [0.5.2] - 2025-02-07

### Changed

- Bundle by default ([#32](https://github.com/gadomski/rustac-py/pull/32))

## [0.5.1] - 2025-02-07

### Added

- More wheels ([#28](https://github.com/gadomski/rustac-py/pull/28))

## [0.5.0] - 2025-02-06

### Changed

- `search` and `search_to` are now async ([#24](https://github.com/gadomski/rustac-py/pull/24))

## [0.4.0] - 2025-01-13

### Added

- DuckDB client ([#15](https://github.com/gadomski/rustac-py/pull/15))

### Changed

- `read` and `write` are now async ([#18](https://github.com/gadomski/rustac-py/pull/18))

## [0.3.0] - 2024-11-21

### Removed

- Validation, pending <https://github.com/stac-utils/stac-rs/issues/517>

### Changed

- Moved out of the <https://github.com/stac-utils/stac-rs> into <https://github.com/gadomski/rustac-py>

## [0.2.2] - 2024-10-22

### Added

- Send user agent when searching ([#487](https://github.com/stac-utils/stac-rs/pull/487))

## [0.2.1] - 2024-10-21

### Added

- More wheels ([#481](https://github.com/stac-utils/stac-rs/pull/481))

## [0.2.0] - 2024-10-19

### Added

- `version` ([#476](https://github.com/stac-utils/stac-rs/pull/476))

### Changed

- Moved docstrings to stub file ([#468](https://github.com/stac-utils/stac-rs/pull/468))

### Removed

- `pystac` ([#468](https://github.com/stac-utils/stac-rs/pull/468))

## [0.1.3] - 2024-10-17

### Added

- Experimental DuckDB 🦆 search on **stac-geoparquet** files ([#458](https://github.com/stac-utils/stac-rs/pull/458))

## [0.1.2] - 2024-09-22

### Changed

- Return the item count from `search_to` ([#426](https://github.com/stac-utils/stac-rs/pull/426))

## [0.1.1] - 2024-09-21

### Added

- Extension module feature

### Changed

- Use Github Pages for docs ([#420](https://github.com/stac-utils/stac-rs/pull/420))

## [0.1.0] - 2024-09-20

### Added

- `migrate_href` ([#334](https://github.com/stac-utils/stac-rs/pull/334))
- `search` and `search_to` ([#387](https://github.com/stac-utils/stac-rs/pull/387))
- `read`, `write`, and `pystac` ([#418](https://github.com/stac-utils/stac-rs/pull/418))

## [0.0.3] - 2024-08-29

### Added

- `migrate` ([#309](https://github.com/stac-utils/stac-rs/pull/309))
- `validate` and docs ([#307](https://github.com/stac-utils/stac-rs/pull/307))

## [0.0.2] - 2024-08-28

Non-functional release to fix releasing from Github actions.

## [0.0.1] - 2024-08-28

Initial release.

[Unreleased]: https://github.com/stac-utils/rustac-py/compare/v0.9.2...main
[0.9.2]: https://github.com/stac-utils/rustac-py/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/stac-utils/rustac-py/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/stac-utils/rustac-py/compare/v0.8.4...v0.9.0
[0.8.4]: https://github.com/stac-utils/rustac-py/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/stac-utils/rustac-py/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/stac-utils/rustac-py/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/stac-utils/rustac-py/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/stac-utils/rustac-py/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/stac-utils/rustac-py/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/stac-utils/rustac-py/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/stac-utils/rustac-py/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/stac-utils/rustac-py/compare/v0.5.9...v0.6.0
[0.5.9]: https://github.com/stac-utils/rustac-py/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/stac-utils/rustac-py/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/stac-utils/rustac-py/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/stac-utils/rustac-py/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/stac-utils/rustac-py/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/stac-utils/rustac-py/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/stac-utils/rustac-py/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/stac-utils/rustac-py/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/stac-utils/rustac-py/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/stac-utils/rustac-py/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/stac-utils/rustac-py/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/stac-utils/rustac-py/releases/tag/v0.3.0
[0.2.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.2.1...python-v0.2.2
[0.2.1]: https://github.com/stac-utils/stac-rs/compare/python-v0.2.0...python-v0.2.1
[0.2.0]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.3...python-v0.2.0
[0.1.3]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.2...python-v0.1.3
[0.1.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.1...python-v0.1.2
[0.1.1]: https://github.com/stac-utils/stac-rs/compare/python-v0.1.0...python-v0.1.1
[0.1.0]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.3...python-v0.1.0
[0.0.3]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.2...python-v0.0.3
[0.0.2]: https://github.com/stac-utils/stac-rs/compare/python-v0.0.1...python-v0.0.2
[0.0.1]: https://github.com/stac-utils/stac-rs/releases/tag/python-v0.0.1

<!-- markdownlint-disable-file MD024 -->
