use crate::index::Index;
use anyhow::{Context, Result as AnyResult};
use h3o::{CellIndex, LatLng};
use std::io;

/// Read cell indexes from stdin.
pub fn read_cell_indexes() -> impl Iterator<Item = AnyResult<CellIndex>> {
    io::stdin().lines().map(|input| {
        input.context("read line from stdin").and_then(|line| {
            line.trim_end()
                .parse()
                .with_context(|| format!("cannot parse {line} as CellIndex"))
        })
    })
}

pub fn read_indexes() -> impl Iterator<Item = AnyResult<Index>> {
    io::stdin().lines().map(|input| {
        input.context("read line from stdin").and_then(|line| {
            line.trim_end()
                .parse()
                .with_context(|| format!("cannot parse {line} as Index"))
        })
    })
}

pub fn read_coords() -> impl Iterator<Item = AnyResult<LatLng>> {
    io::stdin().lines().map(|input| {
        input.context("read line from stdin").and_then(|line| {
            let parts = line.trim_end().split(' ').collect::<Vec<&str>>();
            let lat = parts[0].parse::<f64>().context("latitude")?;
            let lng = parts[1].parse::<f64>().context("longitude")?;

            LatLng::from_degrees(lat, lng).context("lat/lng")
        })
    })
}
