# overpass-cli

A command line tool for querying OpenStreetMap data using the Overpass API.

> [!IMPORTANT]
> By default, this tool makes requests to [overpass-api.de](https://overpass-api.de/), which is a public server generously offered to the OpenStreetMap community free of charge. It is intended primarily for running manual queries and other low-volume use. Please do not abuse this service by making excessive requests or downloading enormous amounts of data. [See the usage policy for more info](https://dev.overpass-api.de/overpass-doc/en/preface/commons.html).

## Example

This invocation finds nodes tagged `natural=arch` in Utah, and returns them as JSON.

```
$ overpass --area 'Utah' --format json 'node[natural = arch]'
{
  "version": 0.6,
  "generator": "Overpass API 0.7.62.1 084b4234",
  "osm3s": {
    "timestamp_osm_base": "2024-09-13T07:18:29Z",
    "copyright": "The data included in this document is from www.openstreetmap.org. The data is made available under ODbL."
  },
  "elements": [
    {
      "type": "node",
      "id": 759250908,
      "lat": 38.7911745,
      "lon": -109.6089896,
      "tags": {
        "name": "Navajo Arch",
        "natural": "arch"
      }
    },
    ...
  ]
}
```

## Installation

This tool is written in Rust and is published to [crates.io](https://crates.io/crates/overpass-cli). If you have a Rust toolchain installed, you can download and build it by running:

```
$ cargo install overpass-cli
```

You can also clone this repository and run `cargo install --path .` in it.

## Usage

```
Usage: overpass [OPTIONS] [QUERY]

Arguments:
  [QUERY]  OverpassQL query string (when omitted, query is read from STDIN)

Options:
  -f, --format <FORMAT>
          Output format [possible values: xml, json]
  -o, --out <OUTPUT>
          Output type [possible values: ids, skel, body, tags, meta, center, geom]
      --area <SEARCH_STRING>
          Area name to geocode with Nominatim (available as 'area' in the query)
      --bbox <MIN_LON> <MIN_LAT> <MAX_LON> <MAX_LAT>
          Global bounding box (implicitly applies to all statements)
      --timeout <TIMEOUT>
          Timeout in seconds
      --mem <MEM>
          Max memory for query (accepts SI suffixes e.g. 512MB, 2GiB)
      --date <DATE>
          Return results for a time in the past (ISO 8601 format)
      --diff <FROM> <TO>
          Compare results at two different times (ISO 8601 format)
      --adiff <FROM> <TO>
          Like --diff, but returns augmented diff with extra information
      --server <URL>
          Overpass server [default: https://overpass-api.de]
      --nominatim-server <URL>
          Nominatim server (queried when --area is used) [default: https://nominatim.openstreetmap.org]
      --dry-run
          Construct and print query but do not send to server
````

## Maturity

This tool is an early (v0.x) draft. Please feel free to report bugs or request missing features.

## License

Code for this tool is available under the terms of the ISC License. See the LICENSE file for details.
