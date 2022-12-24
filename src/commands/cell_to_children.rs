//! Expose [`CellIndex::children`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, Resolution};
use std::{cmp, io};

/// Converts an index into its descendants.
///
/// This command generates the hierarchical children of a cell index at the
/// specified resolution. If the specified resolution is less than or equal to
/// the resolution of the index, only the given cell is returned.
#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Converts descendants from this index.
    #[arg(short, long)]
    parent: CellIndex,

    /// Resolution, if less than PARENT's resolution only PARENT is printed.
    #[arg(short, long, default_value_t = Resolution::Zero)]
    resolution: Resolution,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `cellToChildren` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let resolution = cmp::max(args.parent.resolution(), args.resolution);
    let indexes = args.parent.children(resolution);

    match args.format {
        Format::Text => {
            for index in indexes {
                println!("{index}");
            }
        }
        Format::Json => {
            let mut stdout = io::stdout().lock();
            let indexes = indexes
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            serde_json::to_writer(&mut stdout, &indexes)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}
