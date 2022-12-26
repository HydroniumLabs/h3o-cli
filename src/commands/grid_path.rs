//! Expose [`CellIndex::grid_path_cells`]

use anyhow::{ensure, Context, Result as AnyResult};
use clap::{ArgGroup, Parser, ValueEnum};
use either::Either;
use h3o::CellIndex;
use std::{collections::HashSet, io};

/// Compute the path between the given cell indexes.
#[derive(Parser, Debug)]
#[command(group(ArgGroup::new("ll")
    .args(["source", "destination"])
    .multiple(true)
    .requires_all(["source", "destination"]))
)]
pub struct Args {
    /// The starting point, must be paired with `-d/--destination`.
    #[arg(short, long)]
    source: Option<CellIndex>,

    /// The destination point, must be paired with `-s/--source`.
    #[arg(short, long)]
    destination: Option<CellIndex>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `gridPath` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes =
        if let (Some(src), Some(dst)) = (args.source, args.destination) {
            vec![src, dst]
        } else {
            crate::io::read_cell_indexes()?
        };
    ensure!(indexes.len() >= 2, "not enough cell indexes");

    let path = indexes
        .windows(2)
        .flat_map(|segment| match segment[0].grid_path_cells(segment[1]) {
            Ok(iter) => Either::Right(iter),
            Err(err) => Either::Left(std::iter::once(Err(err))),
        })
        .collect::<Result<HashSet<_>, _>>()
        .context("compute paths")?;

    match args.format {
        Format::Text => {
            for index in path {
                println!("{index}");
            }
        }
        Format::Json => {
            let mut stdout = io::stdout().lock();
            let path = path
                .into_iter()
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            serde_json::to_writer(&mut stdout, &path)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}
