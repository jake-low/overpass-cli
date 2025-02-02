# Changelog

All notable changes to this project will be documented in this file.

Versioning of this project adheres to the [Semantic Versioning](https://semver.org/spec/v2.0.0.html) spec.

## [0.2.0]

- Added `--area` argument to geocode a search string with Nominatim and then
  make the result available for use in the query as the default `area` value.
- Add `User-Agent` header to all outgoing HTTP requests.

## [0.1.1]

- Fixed a bug where `--date`, `--diff`, and `--adiff` arguments weren't
  correctly quoted, requiring the user to wrap them in quotes themselves
  to use them.

## [0.1.0]

Initial release.

[0.2.0]: https://github.com/jake-low/overpass-cli/releases/tag/v0.2.0
[0.1.1]: https://github.com/jake-low/overpass-cli/releases/tag/v0.1.1
[0.1.0]: https://github.com/jake-low/overpass-cli/releases/tag/v0.1.0
