//! Expose [`ToGeo::to_geom`]

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use geojson::Feature;
use h3o::{geom::ToGeo, CellIndex};
use kml::Kml;

/// Converts indexes to (multi)polygon.
///
/// All indexes must have the same resolution.
#[derive(Parser, Debug)]
pub struct Args {
    /// Cell index.
    #[arg(short, long)]
    index: Option<CellIndex>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Geojson)]
    format: Format,

    /// Prettify the output (GeoJSON only)
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Geojson,
    Kml,
}

/// Run the `cellToPolygon` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.index)
        .collect::<AnyResult<Vec<_>>>()?;

    match args.format {
        Format::Geojson => {
            let geometry = indexes.to_geojson().context("compute GeoJSON")?;
            let feature = Feature {
                bbox: None,
                geometry: Some(geometry),
                id: None,
                properties: None,
                foreign_members: None,
            };
            crate::json::print(&feature, args.pretty)?;
        }
        Format::Kml => {
            let style_id = "lineStyle1";
            let style = kml::types::Style {
                id: Some(style_id.to_owned()),
                line: Some(kml::types::LineStyle {
                    id: Some("lineStyle2".to_owned()),
                    color: "ff0000ff".to_owned(),
                    width: 2.,
                    ..kml::types::LineStyle::default()
                }),
                ..kml::types::Style::default()
            };

            let elements = vec![
                Kml::Style(style),
                crate::kml::polygons(
                    indexes.to_geom(true).context("compute polygons")?,
                    style_id,
                ),
            ];

            crate::kml::print_document(
                "H3 Geometry".to_owned(),
                "Generated by cellToPolygon".to_owned(),
                elements,
            )?;
        }
    }

    Ok(())
}
