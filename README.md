# h3o-cli — A CLI app exposing the h3o API for scripting

[![Crates.io](https://img.shields.io/crates/v/h3o-cli.svg)](https://crates.io/crates/h3o-cli)
[![Docs.rs](https://docs.rs/h3o-cli/badge.svg)](https://docs.rs/h3o-cli)
[![CI Status](https://github.com/HydroniumLabs/h3o-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/HydroniumLabs/h3o-cli/actions)
[![License](https://img.shields.io/badge/license-BSD-green)](https://opensource.org/licenses/BSD-3-Clause)

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

## Usage

Each subcommand comes with its own help through `-h/--help`.

There are two classes of output format for the commands:
- text format (text and JSON)
- geo format (KML and GeoJSON)

Most of the commands can either take a single input from the CLI options or a
list of input from `stdin`.

Plain text output can be directly used as input for others, allowing command
pipelines.

For geo output:
- `cellToLatLng` returns the center (`Point`) of each index
- `cellToBoundary` returns the outline (`LineString`) of each index
- `cellToPolygon` returns the shape (`Polygon`) of contiguous indexes.

## Examples

Prints information (coordinates, area, …) about on given cell:
```text
h3o-cli cellInfo -i 844c001ffffffff
```

Decodes an index into its components:
```text
h3o-cli indexDecode -i 21b1fb4644920fff
```

Creates a GeoJSON file containing the cell center points of all of the
resolution 9 hexagons covering Uber HQ and the surrounding region of San
Francisco.
```text
h3o-cli cellToChildren --parent 86283082fffffff --resolution 9 \
    | h3o-cli cellToLatLng -f geojson > uber9pts.geojson
```

Generates the set of indexes that cover Paris at resolution 11 and save the
compacted result in `cells.txt`.
```text
h3o-cli geomToCells -r 11 -f geojson < paris.geojson | h3o-cli compact > cells.txt
```

Prints the indexes from the 2-ring around `89283082ed7ffff`.
```text
h3o-cli gridDisk -o 89283082ed7ffff -r 2
```

At resolution 7, prints the grid path that goes through a bunch of French cities
and return the resulting KML.
```text
h3o-cli latLngToCell -r 7 < cities-center.txt \
    | h3o-cli gridPath \
    | h3o-cli cellToBoundary -f kml
```
