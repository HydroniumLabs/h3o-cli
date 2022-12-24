use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// An [`h3o::CellIndex`] that serialize as a string (for JSON/H3 compat').
pub struct CellIndex(h3o::CellIndex);

impl From<h3o::CellIndex> for CellIndex {
    fn from(value: h3o::CellIndex) -> Self {
        Self(value)
    }
}

impl From<CellIndex> for h3o::CellIndex {
    fn from(value: CellIndex) -> Self {
        value.0
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

impl<'de> Deserialize<'de> for CellIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CellIndexVisitor;

        impl<'de> Visitor<'de> for CellIndexVisitor {
            type Value = CellIndex;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str("the hexstring of a cell index")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value
                    .parse::<h3o::CellIndex>()
                    .map(Into::into)
                    .map_err(|err| E::custom(err.to_string()))
            }
        }

        deserializer.deserialize_str(CellIndexVisitor)
    }
}
