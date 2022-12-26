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
    CellToBoundary(commands::cell_to_boundary::Args),
    CellToChildren(commands::cell_to_children::Args),
    CellToLatLng(commands::cell_to_latlng::Args),
    CellToLocalIj(commands::cell_to_local_ij::Args),
    GridDisk(commands::grid_disk::Args),
    IndexDecode(commands::index_decode::Args),
    LatLngToCell(commands::latlng_to_cell::Args),
}

fn main() -> AnyResult<()> {
    match Args::parse().command {
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
        Command::IndexDecode(args) => {
            commands::index_decode::run(&args)?;
        }
        Command::GridDisk(args) => {
            commands::grid_disk::run(&args)?;
        }
        Command::LatLngToCell(args) => {
            commands::latlng_to_cell::run(&args)?;
        }
    };

    Ok(())
}
