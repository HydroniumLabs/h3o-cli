# h3o-cli — A CLI app exposing most of the h3o API for scripting

## How to install

### Pre-compiled binaries

You can download a pre-compiled executable for Linux, MacOS and Windows
operating systems from the
[release page](https://github.com/HydroniumLabs/h3o-cli/releases/), then you
should copy that executable to a location from your `$PATH` env.

### Build Manually

If you prefer to build `h3o-cli` manually, or a pre-compiled executable is not
provided for your platform, then you can build `h3o-cli` from the source:

- [Install Rust](https://www.rust-lang.org/tools/install)
- Run `cargo install h3o-cli`

## Subcommands

Each command comes with its own help through `-h/--help`.

### cellInfo

Prints information (coordinates, area, …) about cell indexes.

```text
h3o-cli cellInfo -i 844c001ffffffff
```

Also work on a list of indexes from stdin:
```text
h3o-cli cellInfo -f json < cells.txt
```

### cellToBoundary

Converts indexes to latitude/longitude cell boundaries in degrees.

Outputs plain text boundary for the specified cell.
```text
h3o-cli cellToBoundary -i 85283473fffffff
```

Same, but formatted as GeoJSON.
```text
h3o-cli cellToBoundary -i 85283473fffffff -f geojson
```

Creates the KML file `cells.kml` containing the cell boundaries for all of the
cell indexes contained in the file `indexes.txt`.
```text
h3o-cli cellToBoundary -f kml < indexes.txt > cells.kml
```

## cellToChildren

Converts an index into its descendants.

Outputs all of the resolution 3 descendants of cell `820ceffffffffff` as JSON.
```text
h3o-cli cellToChildren --parent 820ceffffffffff --resolution 3 -f json
```

Outputs the cell boundaries of all of the resolution 4 descendants of cell
`820ceffffffffff` as a KML file (redirected to `cells.kml`).
```text
h3o-cli cellToChildren --parent 820ceffffffffff --resolution 4 \
    | h3o-cli cellToBoundary -f kml > cells.kml
```

## cellToLatLng

Converts an index into its descendants.

Outputs plain text cell center points for the H3 indexes contained in the file
`indexes.txt`
```text
h3o-cli cellToLatLng < indexes.txt
```

Creates the KML file `cells.kml` containing the cell center points for all of
the H3 indexes contained in the file `indexes.txt`.
```text
 h3o-cli cellToLatLng -f kml < indexes.txt > cells.kml
```

Creates a GeoJSON file containing the cell center points of all of the
resolution 9 hexagons covering Uber HQ and the surrounding region of
San Francisco.
```text
h3o-cli cellToChildren --parent 86283082fffffff --resolution 9 \
    | h3o-cli cellToLatLng -f geojson > uber9pts.geojson
```

## cellToLocalIj

Converts indexes to local IJ coordinates.

Get the local IJ coordinates, anchored at `8f1fb46622d8591`, of
`8f1fb464492001a`.
```text
h3o-cli cellToLocalIj -o 8f1fb46622d8591 -i 8f1fb464492001a
```

Returns `"NA"` (`null` in JSON) when the coordinates cannot be computed.
```text
h3o-cli cellToLocalIj -o 861fb4667ffffff -i 86283082fffffff
```

## gridDisk

Print cell indexes `radius` distance away from the origin.

Print the indexes from the 2-ring around `89283082ed7ffff`.
```text
h3o-cli gridDisk -o 89283082ed7ffff -r 2
```

You can also print the distances of each indexes from the origin.
```text
h3o-cli gridDisk -r 3 -f json -d < cells.txt
```

## indexDecode

Decode h3o indexes into components.

By default a compact version is printed:
```text
h3o-cli indexDecode -i 124734a9ffffffff
```

But a more verbose version is also available.
```text
h3o-cli indexDecode -i 89283082ed7ffff -f pretty
```

Last but not least, JSON output is also supported.
```text
h3o-cli indexDecode --format json < cells.txt
```

# latLngToCell

Converts from lat/lng coordinates to cell indexes.

Get the cell index that contains the given coordinate at resolution 11.
```text
h3o-cli latLngToCell -r 11 --lat 48.85455798312344 --lng 2.3730553730188952
```

You can also provide a list of coordinate (one per line, latitude first then
longitude after a single space).
```text
h3o-cli latLngToCell -r 11 -f json < coords.txt
```
