use anyhow::{Context, Result as AnyResult};
use serde::{Serialize, Serializer};
use std::io;

/// Output the value as JSON on `stdout`.
///
/// If `pretty` is true the output is pretty-printed.
pub fn print<T>(value: &T, pretty: bool) -> AnyResult<()>
where
    T: ?Sized + Serialize,
{
    let mut stdout = io::stdout().lock();
    if pretty {
        serde_json::to_writer_pretty(&mut stdout, value)
            .context("write pretty JSON to stdout")
    } else {
        serde_json::to_writer(&mut stdout, value)
            .context("write JSON to stdout")
    }
}

// -----------------------------------------------------------------------------

/// An [`h3o::CellIndex`] that serialize as a string (for JSON/H3 compat').
pub struct CellIndex(h3o::CellIndex);

impl From<h3o::CellIndex> for CellIndex {
    fn from(value: h3o::CellIndex) -> Self {
        Self(value)
    }
}

impl Serialize for CellIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
