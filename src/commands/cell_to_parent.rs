//! Expose [`CellIndex::parent`]

use anyhow::Result as AnyResult;
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, Resolution};
use serde::Serialize;

/// Converts an index into its parent.
///
/// This command generates the hierarchical parent of a cell index at the
/// specified resolution. If the specified resolution is greater than or equal to
/// the resolution of the index, only the given cell is returned.
#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Converts parents from this index.
    #[arg(short, long)]
    child: Option<CellIndex>,

    /// Resolution, if greater than PARENT's resolution only PARENT is printed.
    #[arg(short, long, default_value_t = Resolution::Zero)]
    resolution: Resolution,

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

/// Run the `cellToParent` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.child)
        .map(|child| child.map(|child| (child, child.parent(args.resolution))));

    match args.format {
        Format::Text => {
            for result in indexes {
                let (child, parent) = result?;
                println!("{}", parent.unwrap_or(child));
            }
        }
        Format::Json => {
            #[derive(Serialize)]
            struct ChildParent {
                child: crate::json::CellIndex,
                parent: Option<crate::json::CellIndex>,
            }

            let indexes = indexes
                .map(|result| {
                    result.map(|(child, parent)| ChildParent {
                        child: child.into(),
                        parent: parent.map(Into::into),
                    })
                })
                .collect::<AnyResult<Vec<_>>>()?;

            crate::json::print(&indexes, args.pretty)?;
        }
    }

    Ok(())
}
