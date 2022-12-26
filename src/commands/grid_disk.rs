//! Expose [`CellIndex::grid_disk`].

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::CellIndex;
use serde::Serialize;
use std::io;

/// Print cell indexes `radius` distance away from the origin.
///
/// The command reads cell indexes from stdin until EOF and outputs
/// the cell indexes within k-ring `radius` to stdout.
#[derive(Parser, Debug)]
pub struct Args {
    /// Cell index.
    #[arg(short, long)]
    origin: Option<CellIndex>,

    /// Radius (in hexagons).
    #[arg(short, long)]
    radius: u32,

    /// Also return the distance from the origin.
    #[arg(short, long, default_value_t = false)]
    distance: bool,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `gridDisk` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = if let Some(index) = args.origin {
        vec![index]
    } else {
        crate::io::read_cell_indexes()?
    };

    match args.format {
        Format::Text => disks_to_text(&indexes, args.radius, args.distance),
        Format::Json => disks_to_json(&indexes, args.radius, args.distance)?,
    }

    Ok(())
}

/// Print disks as plain text.
fn disks_to_text(indexes: &[CellIndex], radius: u32, with_distance: bool) {
    let disks = indexes
        .iter()
        .copied()
        .flat_map(|index| index.grid_disk_distances_safe(radius));

    if with_distance {
        for (index, distance) in disks {
            println!("{index} {distance}");
        }
    } else {
        for (index, _) in disks {
            println!("{index}");
        }
    }
}

/// Print disks as JSON.
fn disks_to_json(
    indexes: &[CellIndex],
    radius: u32,
    with_distance: bool,
) -> AnyResult<()> {
    let mut stdout = io::stdout().lock();
    let disks = indexes.iter().copied().map(|index| {
        index
            .grid_disk_distances_safe(radius)
            .map(|(index, distance)| {
                (crate::json::CellIndex::from(index), distance)
            })
    });

    if with_distance {
        #[derive(Serialize)]
        struct Neighbor {
            index: crate::json::CellIndex,
            distance: u32,
        }
        let disks = disks
            .map(|disk| {
                disk.map(|(index, distance)| Neighbor { index, distance })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        serde_json::to_writer(&mut stdout, &disks)
            .context("write JSON to stdout")
    } else {
        let disks = disks
            .map(|disk| disk.map(|(index, _)| index).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        serde_json::to_writer(&mut stdout, &disks)
            .context("write JSON to stdout")
    }
}
