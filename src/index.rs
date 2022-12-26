use anyhow::{Context, Error as AnyError};
use h3o::{CellIndex, DirectedEdgeIndex, VertexIndex};
use std::str::FromStr;

/// An h3o index.
#[derive(Debug, Clone, Copy)]
pub enum Index {
    Cell(CellIndex),
    DirectedEdge(DirectedEdgeIndex),
    Vertex(VertexIndex),
}

impl FromStr for Index {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<CellIndex>()
            .map(Index::Cell)
            .or_else(|_| {
                s.parse::<DirectedEdgeIndex>().map(Index::DirectedEdge)
            })
            .or_else(|_| s.parse::<VertexIndex>().map(Index::Vertex))
            .context("invalid h3o index")
    }
}
