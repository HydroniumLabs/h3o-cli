//! Expose [`CellIndex::compact`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::CellIndex;

/// Compact the given set of indexes (from stdin).
///
/// All indexes must have the same resolution.
#[derive(Parser, Debug)]
pub struct Args {
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

/// Run the `compact` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let mut indexes =
        crate::io::read_cell_indexes().collect::<AnyResult<Vec<_>>>()?;

    CellIndex::compact(&mut indexes).context("compaction")?;
    match args.format {
        Format::Text => {
            for index in indexes {
                println!("{index}");
            }
        }
        Format::Json => {
            let compacted = indexes
                .into_iter()
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            crate::json::print(&compacted, args.pretty)?;
        }
    }

    Ok(())
}
