//! Expose [`CellIndex::to_local_ij`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, LocalIJ};
use serde::Serialize;
use std::io;

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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `cellToLatLng` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = if let Some(index) = args.index {
        vec![index]
    } else {
        crate::io::read_cell_indexes()?
    };
    let coords = indexes
        .iter()
        .copied()
        .map(|index| index.to_local_ij(args.origin).ok());

    match args.format {
        Format::Text => local_ij_to_text(coords),
        Format::Json => local_ij_to_json(coords)?,
    }

    Ok(())
}

/// Print local IJ coordinates as plain text.
fn local_ij_to_text(coords: impl IntoIterator<Item = Option<LocalIJ>>) {
    for coord in coords {
        coord.map_or_else(
            || println!("NA"),
            |coord| println!("{} {}", coord.i(), coord.j()),
        );
    }
}

/// Print local IJ coordinates as JSON.
fn local_ij_to_json(
    coords: impl IntoIterator<Item = Option<LocalIJ>>,
) -> AnyResult<()> {
    #[derive(Serialize)]
    struct CoordIJ {
        i: i32,
        j: i32,
    }
    let mut stdout = io::stdout().lock();
    let coords = coords
        .into_iter()
        .map(|item| {
            item.map(|coord| CoordIJ {
                i: coord.i(),
                j: coord.j(),
            })
        })
        .collect::<Vec<_>>();

    serde_json::to_writer(&mut stdout, &coords).context("write JSON to stdout")
}
