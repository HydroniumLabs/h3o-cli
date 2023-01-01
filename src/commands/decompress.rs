//! Expose [`thc::decompress`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use std::io::{self, Read};

/// Decompress and print the cell indexes from the compressed input on stdin.
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

/// Run the `cellToPolygon` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let mut bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .context("read bytes from stdin")?;
    let indexes = thc::decompress(bytes.as_slice());

    match args.format {
        Format::Text => {
            for index in indexes {
                println!("{}", index.context("decompress")?);
            }
        }
        Format::Json => {
            let indexes = indexes
                .map(|index| index.map(Into::into))
                .collect::<Result<Vec<crate::json::CellIndex>, _>>()
                .context("decompress")?;
            crate::json::print(&indexes, args.pretty)?;
        }
    }

    Ok(())
}
