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
    };

    Ok(())
}
