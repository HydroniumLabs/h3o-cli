//! Expose [`CellIndex::grid_path_cells`]

use anyhow::{Context, Result as AnyResult, ensure};
use clap::{ArgGroup, Parser, ValueEnum};
use either::Either;
use h3o::CellIndex;

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

    /// Prettify the output (JSON only).
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
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
            crate::io::read_cell_indexes().collect::<AnyResult<Vec<_>>>()?
        };
    ensure!(indexes.len() >= 2, "not enough cell indexes");

    let mut path = indexes
        .windows(2)
        .flat_map(|segment| match segment[0].grid_path_cells(segment[1]) {
            Ok(iter) => Either::Right(iter),
            Err(err) => Either::Left(std::iter::once(Err(err))),
        })
        .collect::<Result<Vec<_>, _>>()
        .context("compute paths")?;
    path.dedup();

    match args.format {
        Format::Text => {
            for index in path {
                println!("{index}");
            }
        }
        Format::Json => {
            let path = path
                .into_iter()
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            crate::json::print(&path, args.pretty)?;
        }
    }

    Ok(())
}
