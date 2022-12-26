//! Expose [`CellIndex::compact`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::CellIndex;
use std::io;

/// Compact the given set of indexes (from stdin).
///
/// All indexes must have the same resolution.
#[derive(Parser, Debug)]
pub struct Args {
    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `compact` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::io::read_cell_indexes()?;

    let compacted = CellIndex::compact(indexes).context("compaction")?;
    match args.format {
        Format::Text => {
            for index in compacted {
                println!("{index}");
            }
        }
        Format::Json => {
            let mut stdout = io::stdout().lock();
            let compacted = compacted
                .into_iter()
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            serde_json::to_writer(&mut stdout, &compacted)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}
