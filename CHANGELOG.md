# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.6-beta.0] - 2025-02-22

### Added

- Search to an arrow table ([#54](https://github.com/stac-utils/stacrs/pull/54))

### Changed

- Include **libduckdb** in wheels ([#52](https://github.com/stac-utils/stacrs/pull/52))

## [0.5.5] - 2025-02-20

### Fixed

- Removed tracing subscriber to fix CLI ([#49](https://github.com/stac-utils/stacrs/pull/49))

## [0.5.4] - 2025-02-19

### Added

- CLI ([#46](https://github.com/stac-utils/stacrs/pull/46))
- Config args to DuckDB client ([#42](https://github.com/stac-utils/stacrs/pull/42))

## [0.5.3] - 2025-02-07

### Changed

- Use only abi3 wheels ([#36](https://github.com/gadomski/stacrs/pull/36))

> [!WARNING]
> All versions older than v0.5.3 were deleted from PyPI, but some tags still exist on this repo.
> See <https://github.com/gadomski/stacrs/discussions/37> for more.

## [0.5.2] - 2025-02-07

### Changed

- Bundle by default ([#32](https://github.com/gadomski/stacrs/pull/32))

## [0.5.1] - 2025-02-07

### Added

- More wheels ([#28](https://github.com/gadomski/stacrs/pull/28))

## [0.5.0] - 2025-02-06

### Changed

- `search` and `search_to` are now async ([#24](https://github.com/gadomski/stacrs/pull/24))

## [0.4.0] - 2025-01-13

### Added

- DuckDB client ([#15](https://github.com/gadomski/stacrs/pull/15))

### Changed

- `read` and `write` are now async ([#18](https://github.com/gadomski/stacrs/pull/18))

## [0.3.0] - 2024-11-21

### Removed

- Validation, pending <https://github.com/stac-utils/stac-rs/issues/517>

### Changed

- Moved out of the <https://github.com/stac-utils/stac-rs> into <https://github.com/gadomski/stacrs>

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

[Unreleased]: https://github.com/gadomski/stacrs/compare/v0.5.6-beta.0...main
[0.5.6-beta.0]: https://github.com/gadomski/stacrs/compare/v0.5.5...v0.5.6-beta.0
[0.5.5]: https://github.com/gadomski/stacrs/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/gadomski/stacrs/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/gadomski/stacrs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/gadomski/stacrs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/gadomski/stacrs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/gadomski/stacrs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/gadomski/stacrs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/gadomski/stacrs/releases/tag/v0.3.0
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
