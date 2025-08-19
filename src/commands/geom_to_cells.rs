//! Expose [`ToCells::to_cells`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use geo_types::Geometry;
use h3o::{
    LatLng, Resolution,
    geom::{ContainmentMode, PlotterBuilder, TilerBuilder},
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
    Covers,
}

impl From<Mode> for ContainmentMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::ContainsCentroid => Self::ContainsCentroid,
            Mode::ContainsBoundary => Self::ContainsBoundary,
            Mode::IntersectsBoundary => Self::IntersectsBoundary,
            Mode::Covers => Self::Covers,
        }
    }
}

/// Run the `cellToPolygon` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = match args.format {
        Format::Geojson => {
            let geojson = geojson::GeoJson::from_reader(io::stdin())
                .context("read GeoJSON")?;
            let geometry =
                Geometry::try_from(geojson).context("invalid geometry")?;

            geometry_to_cells(geometry, args.resolution, args.mode.into())
        }
        Format::Kml => {
            let kml: Kml<f64> =
                KmlReader::from_reader(BufReader::new(io::stdin()))
                    .read()
                    .context("parse KML")?;
            let geometry = crate::kml::to_geometry(kml)
                .context("invalid KML geometry")?
                .context("no KML geometry")?;

            geometry_to_cells(geometry, args.resolution, args.mode.into())
        }
    }?;

    for index in indexes {
        println!("{index}");
    }

    Ok(())
}

fn geometry_to_cells(
    geometry: Geometry<f64>,
    resolution: Resolution,
    mode: ContainmentMode,
) -> AnyResult<HashSet<h3o::CellIndex>> {
    let mut tiler =
        TilerBuilder::new(resolution).containment_mode(mode).build();
    let mut plotter = PlotterBuilder::new(resolution).build();

    match geometry {
        Geometry::Line(line) => {
            plotter.add(line).context("invalid line")?;
            plotter
                .plot()
                .collect::<Result<_, _>>()
                .context("Line plot failed")
        }
        Geometry::LineString(line_string) => {
            plotter
                .add_batch(line_string.lines())
                .context("invalid LineString")?;
            plotter
                .plot()
                .collect::<Result<_, _>>()
                .context("LineString plot failed")
        }
        Geometry::Point(point) => Ok(std::iter::once(
            LatLng::try_from(point.0)
                .context("invalid point")?
                .to_cell(resolution),
        )
        .collect()),
        Geometry::Polygon(polygon) => {
            tiler.add(polygon).context("invalid polygon")?;
            Ok(tiler.into_coverage().collect())
        }
        Geometry::MultiPoint(multi_point) => multi_point
            .iter()
            .map(|point| {
                Ok(LatLng::try_from(point.0)
                    .context("invalid MultiPoint")?
                    .to_cell(resolution))
            })
            .collect::<AnyResult<_>>(),
        Geometry::MultiLineString(multi_line_string) => {
            for line_string in multi_line_string.iter() {
                plotter
                    .add_batch(line_string.lines())
                    .context("invalid MultiLineString")?;
            }
            plotter
                .plot()
                .collect::<Result<_, _>>()
                .context("MultiLineString plot failed")
        }
        Geometry::MultiPolygon(polygons) => {
            tiler.add_batch(polygons).context("invalid MultiPolygon")?;
            Ok(tiler.into_coverage().collect())
        }
        Geometry::Rect(rect) => {
            tiler.add(rect.to_polygon()).context("invalid rectangle")?;
            Ok(tiler.into_coverage().collect())
        }
        Geometry::Triangle(triangle) => {
            tiler
                .add(triangle.to_polygon())
                .context("invalid triangle")?;
            Ok(tiler.into_coverage().collect())
        }
        Geometry::GeometryCollection(geoms) => {
            geoms.into_iter().try_fold(HashSet::new(), |mut acc, geom| {
                let cells = geometry_to_cells(geom, resolution, mode)?;
                acc.extend(cells);
                Ok(acc)
            })
        }
    }
}
