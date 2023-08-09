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
