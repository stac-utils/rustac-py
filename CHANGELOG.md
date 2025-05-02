# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/stac-utils/rustac-py/compare/v0.7.1...main
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
