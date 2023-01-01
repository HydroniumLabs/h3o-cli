//! Expose [`thc::compress`]

use anyhow::{Context, Result as AnyResult};
use clap::Parser;
use std::io;

/// Compress the given set of indexes (from stdin).
#[derive(Parser, Debug)]
pub struct Args;

/// Run the `compact` command.
pub fn run(_args: &Args) -> AnyResult<()> {
    let mut indexes =
        crate::io::read_cell_indexes().collect::<AnyResult<Vec<_>>>()?;
    indexes.sort_unstable();

    let mut stdout = io::stdout().lock();
    thc::compress(&mut stdout, indexes).context("compression")?;

    Ok(())
}
