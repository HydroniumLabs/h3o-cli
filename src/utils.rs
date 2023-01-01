use anyhow::Result as AnyResult;
use either::Either;
use h3o::CellIndex;

/// Get cell indexes, either from a CLI argument or `stdin`.
///
/// First try the CLI arg, and if not set then read from `stdin`.
pub fn get_cell_indexes(
    arg: Option<CellIndex>,
) -> impl Iterator<Item = AnyResult<CellIndex>> {
    arg.map_or_else(
        || Either::Left(crate::io::read_cell_indexes()),
        |index| Either::Right(std::iter::once(Ok(index))),
    )
}
