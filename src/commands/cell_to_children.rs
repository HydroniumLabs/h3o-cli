//! Expose [`CellIndex::children`]

use anyhow::Result as AnyResult;
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, Resolution};
use serde::Serialize;

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
    ancestor: Option<CellIndex>,

    /// Resolution, if less than PARENT's resolution only PARENT is printed.
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

/// Run the `cellToChildren` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.ancestor).map(|parent| {
        parent.map(|parent| (parent, parent.children(args.resolution)))
    });

    match args.format {
        Format::Text => {
            for result in indexes {
                let (_, children) = result?;
                for child in children {
                    println!("{child}");
                }
            }
        }
        Format::Json => {
            #[derive(Serialize)]
            struct ParentChildren {
                parent: crate::json::CellIndex,
                children: Option<Vec<crate::json::CellIndex>>,
            }

            let indexes = indexes
                .map(|result| {
                    result.map(|(parent, children)| {
                        let children =
                            children.map(Into::into).collect::<Vec<_>>();
                        ParentChildren {
                            parent: parent.into(),
                            children: (!children.is_empty())
                                .then_some(children),
                        }
                    })
                })
                .collect::<AnyResult<Vec<_>>>()?;

            crate::json::print(&indexes, args.pretty)?;
        }
    }

    Ok(())
}
