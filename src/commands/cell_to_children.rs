//! Expose [`CellIndex::children`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{CellIndex, Resolution};
use serde::Serialize;
use std::io;

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
    parent: Option<CellIndex>,

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
    let indexes = if let Some(index) = args.parent {
        vec![index]
    } else {
        crate::io::read_cell_indexes()?
    };
    let indexes = indexes
        .into_iter()
        .map(|parent| (parent, parent.children(args.resolution)));

    match args.format {
        Format::Text => {
            for index in indexes.flat_map(|(_, children)| children) {
                println!("{index}");
            }
        }
        Format::Json => {
            #[derive(Serialize)]
            struct ParentChildren {
                parent: crate::json::CellIndex,
                children: Option<Vec<crate::json::CellIndex>>,
            }

            let mut stdout = io::stdout().lock();
            let indexes = indexes
                .map(|(parent, children)| {
                    let children = children.map(Into::into).collect::<Vec<_>>();
                    ParentChildren {
                        parent: parent.into(),
                        children: (!children.is_empty()).then_some(children),
                    }
                })
                .collect::<Vec<_>>();
            serde_json::to_writer(&mut stdout, &indexes)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}
