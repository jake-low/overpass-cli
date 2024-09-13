# overpass-cli

A command line tool for querying OpenStreetMap data using the Overpass API.

## Example

This invocation finds nodes tagged `natural=arch` in a bounding box roughly covering Arches National Park in Utah, and returns them as JSON.

```
$ overpass --bbox -109.723 38.602 -109.469 38.856 --format json 'node[natural = arch]'
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
      --bbox <MIN_LON> <MIN_LAT> <MAX_LON> <MAX_LAT>
          Global bounding box (implicitly applies to all statements)
      --date <DATE>
          Return results for a time in the past (ISO 8601 format)
      --diff <FROM> <TO>
          Compare results at two different times (ISO 8601 format)
      --adiff <FROM> <TO>
          Like --diff, but returns augmented diff with extra information
      --server <URL>
          Server URL [default: https://overpass-api.de]
      --dry-run
          Construct and print query but do not send to server
````

## Status

This tool is an early (v0.x) draft. Please feel free to report bugs or request missing features.

## License

Code for this tool is available under the terms of the ISC License. See the LICENSE file for details.
