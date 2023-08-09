//! Expose [`ToCells::to_cells`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{
    geom::{ContainmentMode, Geometry, ToCells},
    Resolution,
};
use kml::{Kml, KmlReader};
use std::{
    collections::HashSet,
    io::{self, BufReader},
};

/// Converts geometry from stdin into cells at the given resolution.
#[derive(Parser, Debug)]
pub struct Args {
    /// Target resolution.
    #[arg(short, long)]
    resolution: Resolution,

    /// Polyfill mode.
    #[arg(short, long, value_enum, default_value_t = Mode::ContainsCentroid)]
    mode: Mode,

    /// Input format.
    #[arg(short, long, value_enum, default_value_t = Format::Geojson)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Geojson,
    Kml,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Mode {
    ContainsCentroid,
    ContainsBoundary,
    IntersectsBoundary,
}

impl From<Mode> for ContainmentMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::ContainsCentroid => Self::ContainsCentroid,
            Mode::ContainsBoundary => Self::ContainsBoundary,
            Mode::IntersectsBoundary => Self::IntersectsBoundary,
        }
    }
}

/// Run the `cellToPolygon` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let config = h3o::geom::PolyfillConfig::new(args.resolution)
        .containment_mode(args.mode.into());

    let indexes = match args.format {
        Format::Geojson => {
            let geojson = geojson::GeoJson::from_reader(io::stdin())
                .context("read GeoJSON")?;
            let geometry =
                Geometry::try_from(&geojson).context("invalid geometry")?;

            geometry.to_cells(config).collect::<HashSet<_>>()
        }
        Format::Kml => {
            let kml: Kml<f64> =
                KmlReader::from_reader(BufReader::new(io::stdin()))
                    .read()
                    .context("parse KML")?;
            let geometry = Geometry::from_degrees(
                crate::kml::to_geometry(kml)
                    .context("invalid KML geometry")?
                    .context("no KML geometry")?,
            )
            .context("invalid geometry")?;

            geometry.to_cells(config).collect::<HashSet<_>>()
        }
    };

    for index in indexes {
        println!("{index}");
    }

    Ok(())
}
