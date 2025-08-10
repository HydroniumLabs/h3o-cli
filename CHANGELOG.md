# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Possible sections are:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.2.7] - 2025-08-10

### Changed

- bump `h3o` to 0.8
- bump `kml` to 0.10

## [0.2.6] - 2024-11-28

### Changed

- bump `h3o` to 0.7

## [0.2.5] - 2024-10-03

### Added

- support for `covers` mode in `geomToCells`

## [0.2.4] - 2024-04-15

### Added

- the cellToParent command

## [0.2.3] - 2024-03-25

### Changed

- bump deps (`h3o` to `0.6` and replace `thc` by `h3o-zip`)

## [0.2.2] - 2024-01-15

### Changed

- bump deps

## [0.2.1] - 2023-10-26

### Changed

- bump deps to fix GHSA-c827-hfw6-qwvm

## [0.2.0] - 2023-08-09

### Changed

- coordinates order in polygons' loops is no longer stable (it may change
  between two executions), but the relative order is preserved (i.e. they still
  represent the same geometrical object)

### Added

- the polyfill mode for CellToPolygon can be selected with `-m/--mode`

## [0.1.3] - 2023-05-30

### Changed

- bump deps

## [0.1.2] - 2023-04-29

### Changed

- bump deps

## [0.1.1] - 2023-01-15

### Changed

- bump h3o

## [0.1.0] - 2023-01-09

- initial release
