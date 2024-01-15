//! Expose [`CellIndex::to_local_ij`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, LocalIJ};
use serde::Serialize;

/// Converts indexes to local IJ coordinates.
///
/// The command reads H3 indexes from stdin and outputs the corresponding IJ
/// coordinates to stdout, until EOF is encountered. `NA` is printed if the IJ
/// coordinates could not be obtained.
#[derive(Parser, Debug)]
pub struct Args {
    /// The origin (or anchoring) index for the IJ coordinate.
    #[arg(short, long)]
    origin: CellIndex,

    /// Cell index.
    #[arg(short, long)]
    index: Option<CellIndex>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Prettify the output (JSON only).
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `cellToLocalIj` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.index);
    let coords = indexes
        .map(|input| input.map(|index| index.to_local_ij(args.origin).ok()));

    match args.format {
        Format::Text => local_ij_to_text(coords),
        Format::Json => local_ij_to_json(coords, args.pretty),
    }
    .context("cellToLocalIj")?;

    Ok(())
}

/// Print local IJ coordinates as plain text.
fn local_ij_to_text(
    coords: impl IntoIterator<Item = AnyResult<Option<LocalIJ>>>,
) -> AnyResult<()> {
    for coord in coords {
        coord?.map_or_else(
            || println!("NA"),
            |coord| println!("{} {}", coord.coord.i, coord.coord.j),
        );
    }

    Ok(())
}

/// Print local IJ coordinates as JSON.
fn local_ij_to_json(
    coords: impl IntoIterator<Item = AnyResult<Option<LocalIJ>>>,
    pretty: bool,
) -> AnyResult<()> {
    #[derive(Serialize)]
    struct CoordIJ {
        i: i32,
        j: i32,
    }
    let coords = coords
        .into_iter()
        .map(|result| {
            result.map(|value| {
                value.map(|coord| CoordIJ {
                    i: coord.coord.i,
                    j: coord.coord.j,
                })
            })
        })
        .collect::<AnyResult<Vec<_>>>()?;

    crate::json::print(&coords, pretty)
}
