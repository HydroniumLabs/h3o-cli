//! Expose [`LatLng::from`](./struct.LatLng.html#impl-From<CellIndex>-for-LatLng)

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use geojson::{FeatureCollection, GeoJson};
use h3o::{CellIndex, LatLng};
use kml::Kml;

/// Converts indexes to latitude/longitude center coordinates in degrees.
///
/// The command reads H3 indexes from stdin and outputs the corresponding cell
/// center points to stdout, until EOF is encountered.
#[derive(Parser, Debug)]
pub struct Args {
    /// Cell index.
    #[arg(short, long)]
    index: Option<CellIndex>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Prettify the output.
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
    Geojson,
    Kml,
}

/// Run the `cellToLatLng` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.index);

    match args.format {
        Format::Text => latlng_to_text(indexes),
        Format::Json => latlng_to_json(indexes, args.pretty),
        Format::Geojson => latlng_to_geojson(indexes, args.pretty),
        Format::Kml => latlng_to_kml(indexes),
    }
    .context("cellToLatLng")?;

    Ok(())
}

/// Print lat/lng as plain text.
fn latlng_to_text(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
) -> AnyResult<()> {
    for ll in indexes.into_iter().map(|input| input.map(LatLng::from)) {
        let ll = ll?;
        println!("{:.9} {:.9}", ll.lat(), ll.lng());
    }

    Ok(())
}

/// Print lat/lng as JSON.
fn latlng_to_json(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
    pretty: bool,
) -> AnyResult<()> {
    let coords = indexes
        .into_iter()
        .map(|input| input.map(LatLng::from))
        .collect::<AnyResult<Vec<_>>>()?;

    crate::json::print(&coords, pretty)
}

/// Print lat/lng as geojson.
fn latlng_to_geojson(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
    pretty: bool,
) -> AnyResult<()> {
    let indexes = indexes.into_iter().collect::<AnyResult<Vec<_>>>()?;
    let features = crate::geojson::centers(&indexes);
    let geojson = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    });

    crate::json::print(&geojson, pretty)
}

/// Print lat/lng as KML.
fn latlng_to_kml(
    indexes: impl IntoIterator<Item = AnyResult<CellIndex>>,
) -> AnyResult<()> {
    let indexes = indexes.into_iter().collect::<AnyResult<Vec<_>>>()?;
    // Define styles.
    let style = kml::types::Style {
        id: Some("s_circle".to_owned()),
        icon: Some(kml::types::IconStyle{
            scale: 1.1,
            icon: kml::types::Icon {
                href: "http://maps.google.com/mapfiles/kml/shapes/placemark_circle.png".to_owned(),
                ..kml::types::Icon::default()
            },
            hot_spot: Some(kml::types::Vec2 {
                x: 20.,
                y: 2.,
                xunits: kml::types::Units::Pixels,
                yunits: kml::types::Units::Pixels,
            }),
            ..kml::types::IconStyle::default()
        }),
        label: Some(kml::types::LabelStyle{
            color: "ff0000ff".to_owned(),
            scale: 2.,
            ..kml::types::LabelStyle::default()
        }),
        ..kml::types::Style::default()
    };
    let style_hl = kml::types::Style {
        id: Some("s_circle_hl".to_owned()),
        icon: Some(kml::types::IconStyle{
            scale: 1.3,
            icon: kml::types::Icon {
                href: "http://maps.google.com/mapfiles/kml/shapes/placemark_circle.png".to_owned(),
                ..kml::types::Icon::default()
            },
            hot_spot: Some(kml::types::Vec2 {
                x: 20.,
                y: 2.,
                xunits: kml::types::Units::Pixels,
                yunits: kml::types::Units::Pixels,
            }),
            ..kml::types::IconStyle::default()
        }),
        label: Some(kml::types::LabelStyle{
            color: "ff0000ff".to_owned(),
            scale: 2.,
            ..kml::types::LabelStyle::default()
        }),
        ..kml::types::Style::default()
    };
    let style_id = "m_ylw-pushpin";
    let style_map = kml::types::StyleMap {
        id: Some(style_id.to_owned()),
        pairs: vec![
            kml::types::Pair {
                key: "normal".to_owned(),
                style_url: "#s_circle".to_owned(),
                ..kml::types::Pair::default()
            },
            kml::types::Pair {
                key: "highlight".to_owned(),
                style_url: "#s_circle_hl".to_owned(),
                ..kml::types::Pair::default()
            },
        ],
        ..kml::types::StyleMap::default()
    };
    let mut elements = vec![
        Kml::Style(style),
        Kml::Style(style_hl),
        Kml::StyleMap(style_map),
    ];
    elements.append(&mut crate::kml::centers(&indexes, style_id));

    crate::kml::print_document(
        "H3 Geometry".to_owned(),
        "Generated by cellToLatLng".to_owned(),
        elements,
    )
}
