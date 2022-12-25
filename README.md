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
