use anyhow::Result as AnyResult;
use clap::Parser;
use h3o_cli::commands;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
#[command(rename_all = "camelCase")]
enum Command {
    CellInfo(commands::cell_info::Args),
    CellToBoundary(commands::cell_to_boundary::Args),
    CellToChildren(commands::cell_to_children::Args),
    CellToLatLng(commands::cell_to_latlng::Args),
    CellToLocalIj(commands::cell_to_local_ij::Args),
    CellToParent(commands::cell_to_parent::Args),
    CellToPolygon(commands::cell_to_polygon::Args),
    Compact(commands::compact::Args),
    Compress(commands::compress::Args),
    Decompress(commands::decompress::Args),
    GeomToCells(commands::geom_to_cells::Args),
    GridDisk(commands::grid_disk::Args),
    GridPath(commands::grid_path::Args),
    IndexDecode(commands::index_decode::Args),
    LatLngToCell(commands::latlng_to_cell::Args),
    ResolutionInfo(commands::resolution_info::Args),
}

fn main() -> AnyResult<()> {
    match Args::parse().command {
        Command::CellInfo(args) => {
            commands::cell_info::run(&args)?;
        }
        Command::CellToBoundary(args) => {
            commands::cell_to_boundary::run(&args)?;
        }
        Command::CellToChildren(args) => {
            commands::cell_to_children::run(&args)?;
        }
        Command::CellToLatLng(args) => {
            commands::cell_to_latlng::run(&args)?;
        }
        Command::CellToLocalIj(args) => {
            commands::cell_to_local_ij::run(&args)?;
        }
        Command::CellToParent(args) => {
            commands::cell_to_parent::run(&args)?;
        }
        Command::CellToPolygon(args) => {
            commands::cell_to_polygon::run(&args)?;
        }
        Command::Compact(args) => {
            commands::compact::run(&args)?;
        }
        Command::Compress(args) => {
            commands::compress::run(&args)?;
        }
        Command::Decompress(args) => {
            commands::decompress::run(&args)?;
        }
        Command::IndexDecode(args) => {
            commands::index_decode::run(&args)?;
        }
        Command::GeomToCells(args) => {
            commands::geom_to_cells::run(&args)?;
        }
        Command::GridDisk(args) => {
            commands::grid_disk::run(&args)?;
        }
        Command::GridPath(args) => {
            commands::grid_path::run(&args)?;
        }
        Command::LatLngToCell(args) => {
            commands::latlng_to_cell::run(&args)?;
        }
        Command::ResolutionInfo(args) => {
            commands::resolution_info::run(&args)?;
        }
    };

    Ok(())
}
