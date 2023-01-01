//! Expose [`CellIndex::grid_disk`].

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::CellIndex;
use serde::Serialize;

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

    /// Prettify the output (JSON only).
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `gridDisk` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.origin);

    match args.format {
        Format::Text => disks_to_text(indexes, args.radius, args.distance),
        Format::Json => {
            disks_to_json(indexes, args.radius, args.distance, args.pretty)
        }
    }
    .context("gridDisk")?;

    Ok(())
}

/// Print disks as plain text.
fn disks_to_text(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
    radius: u32,
    with_distance: bool,
) -> AnyResult<()> {
    let disks = indexes
        .into_iter()
        .map(|input| input.map(|index| index.grid_disk_distances_safe(radius)));

    if with_distance {
        for disk in disks {
            let disk = disk?;
            for (index, distance) in disk {
                println!("{index} {distance}");
            }
        }
    } else {
        for disk in disks {
            let disk = disk?;
            for (index, _) in disk {
                println!("{index}");
            }
        }
    }

    Ok(())
}

/// Print disks as JSON.
fn disks_to_json(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
    radius: u32,
    with_distance: bool,
    pretty: bool,
) -> AnyResult<()> {
    let disks = indexes.into_iter().map(|input| {
        input.map(|origin| {
            origin
                .grid_disk_distances_safe(radius)
                .map(|(index, distance)| {
                    (crate::json::CellIndex::from(index), distance)
                })
        })
    });

    if with_distance {
        #[derive(Serialize)]
        struct Neighbor {
            index: crate::json::CellIndex,
            distance: u32,
        }
        let disks = disks
            .map(|result| {
                result.map(|disk| {
                    disk.map(|(index, distance)| Neighbor { index, distance })
                        .collect::<Vec<_>>()
                })
            })
            .collect::<AnyResult<Vec<_>>>()?;
        crate::json::print(&disks, pretty)
    } else {
        let disks = disks
            .map(|result| {
                result.map(|disk| {
                    disk.map(|(index, _)| index).collect::<Vec<_>>()
                })
            })
            .collect::<AnyResult<Vec<_>>>()?;
        crate::json::print(&disks, pretty)
    }
}
