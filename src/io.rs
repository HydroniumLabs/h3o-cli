use crate::index::Index;
use anyhow::{Context, Result as AnyResult};
use h3o::CellIndex;
use std::io;

pub fn read_cell_indexes() -> AnyResult<Vec<CellIndex>> {
    let mut indexes = Vec::new();
    let mut input = String::new();
    loop {
        if io::stdin()
            .read_line(&mut input)
            .context("read line from stdin")?
            == 0
        {
            break;
        }
        indexes.push(
            input.trim_end().parse().with_context(|| {
                format!("cannot parse {input} as CellIndex")
            })?,
        );
        input.clear();
    }

    Ok(indexes)
}

// TODO: refactor with read_cell_indexes.
pub fn read_indexes() -> AnyResult<Vec<Index>> {
    let mut indexes = Vec::new();
    let mut input = String::new();
    loop {
        if io::stdin()
            .read_line(&mut input)
            .context("read line from stdin")?
            == 0
        {
            break;
        }
        indexes.push(
            input
                .trim_end()
                .parse()
                .with_context(|| format!("cannot parse {input} as Index"))?,
        );
        input.clear();
    }

    Ok(indexes)
}
