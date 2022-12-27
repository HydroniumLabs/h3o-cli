use anyhow::{Context, Result as AnyResult};
use geo_types::coord;
use h3o::{geom::ToGeo, CellIndex, LatLng};
use kml::{Kml, KmlDocument, KmlWriter};
use maplit::hashmap;
use std::io;

/// Return KML Placemarks representing the indexes' boundaries.
pub fn boundaries(indexes: &[CellIndex], style: &str) -> Vec<Kml> {
    indexes
        .iter()
        .map(|index| {
            let (linestring, _) =
                index.to_geom(true).expect("infaillible").into_inner();
            let geometry = kml::types::LineString {
                coords: linestring.0.into_iter().map(Into::into).collect(),
                tessellate: true,
                ..kml::types::LineString::default()
            };
            let placemark = kml::types::Placemark {
                name: Some(index.to_string()),
                attrs: hashmap! {
                    "styleUrl".to_owned() => format!("#{style}"),
                },
                geometry: Some(kml::types::Geometry::LineString(geometry)),
                ..kml::types::Placemark::default()
            };
            Kml::Placemark(placemark)
        })
        .collect()
}

/// Return KML Placemarks representing the indexes centers.
pub fn centers(indexes: &[CellIndex], style: &str) -> Vec<Kml> {
    indexes
        .iter()
        .copied()
        .map(|index| {
            let ll = LatLng::from(index);
            let geometry = kml::types::Point {
                coord: coord! {x: ll.lng_degrees(), y: ll.lat_degrees()}.into(),
                altitude_mode: kml::types::AltitudeMode::RelativeToGround,
                ..kml::types::Point::default()
            };
            let placemark = kml::types::Placemark {
                name: Some(index.to_string()),
                attrs: hashmap! {
                    "styleUrl".to_owned() => format!("#{style}"),
                },
                geometry: Some(kml::types::Geometry::Point(geometry)),
                ..kml::types::Placemark::default()
            };
            Kml::Placemark(placemark)
        })
        .collect()
}

/// Print the given KML elements on stdout.
///
/// # Errors
///
/// Returns an error if an I/O error occurs while printing the KML.
pub fn print_document(
    name: String,
    description: String,
    elements: Vec<Kml>,
) -> AnyResult<()> {
    let document = KmlDocument::<f64> {
        version: kml::KmlVersion::V22,
        attrs: hashmap! {
            "xmlns".to_owned()      => "http://www.opengis.net/kml/2.2".to_owned(),
            "xmlns:gx".to_owned()   => "http://www.google.com/kml/ext/2.2".to_owned(),
            "xmlns:kml".to_owned()  => "http://www.opengis.net/kml/2.2".to_owned(),
            "xmlns:atom".to_owned() => "http://www.w3.org/2005/Atom".to_owned(),
        },
        elements: vec![Kml::Folder {
            attrs: hashmap! {
                "name".to_owned()        => name,
                "description".to_owned() => description,
            },
            elements,
        }],
    };

    let mut stdout = io::stdout().lock();
    let mut writer = KmlWriter::from_writer(&mut stdout);
    let kml = Kml::KmlDocument(document);

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    writer.write(&kml).context("write KML to stdout")?;

    Ok(())
}

/// Return a KML Placemark representing the indexes' polygons.
pub fn polygons(polygons: geo_types::MultiPolygon, style: &str) -> Kml {
    let geometries = kml::types::MultiGeometry::new(
        polygons
            .into_iter()
            .map(|polygon| {
                let mut polygon = kml::types::Polygon::from(polygon);
                polygon.tessellate = true;
                kml::types::Geometry::Polygon(polygon)
            })
            .collect(),
    );

    let placemark = kml::types::Placemark {
        attrs: hashmap! {
            "styleUrl".to_owned() => format!("#{style}"),
        },
        geometry: Some(kml::types::Geometry::MultiGeometry(geometries)),
        ..kml::types::Placemark::default()
    };
    Kml::Placemark(placemark)
}

/// Extract geometries from a KML element.
pub fn to_geometry(kml: Kml) -> AnyResult<Option<geo_types::Geometry>> {
    Ok(match kml {
        Kml::KmlDocument(document) => {
            Some(geo_types::Geometry::GeometryCollection(
                geo_types::GeometryCollection(
                    document
                        .elements
                        .into_iter()
                        .filter_map(|element| to_geometry(element).transpose())
                        .collect::<Result<Vec<_>, _>>()
                        .context("invalid geometry in document/folder")?,
                ),
            ))
        }
        Kml::Point(point) => Some(geo_types::Geometry::Point(point.into())),
        Kml::Location(location) => Some(geo_types::Geometry::Point(
            (location.longitude, location.latitude).into(),
        )),
        Kml::LineString(line) => {
            Some(geo_types::Geometry::LineString(line.into()))
        }
        Kml::LinearRing(ring) => {
            Some(geo_types::Geometry::LineString(ring.into()))
        }
        Kml::Polygon(polygon) => {
            Some(geo_types::Geometry::Polygon(polygon.into()))
        }
        Kml::MultiGeometry(geometries) => {
            Some(geo_types::Geometry::GeometryCollection(
                geometries.try_into().context("invalid multigeometry")?,
            ))
        }
        Kml::Placemark(placemark) => placemark
            .geometry
            .map(TryInto::try_into)
            .transpose()
            .context("invalid geometry in placemark")?,
        Kml::Document { elements, .. } | Kml::Folder { elements, .. } => {
            Some(geo_types::Geometry::GeometryCollection(
                geo_types::GeometryCollection(
                    elements
                        .into_iter()
                        .filter_map(|element| to_geometry(element).transpose())
                        .collect::<Result<Vec<_>, _>>()
                        .context("invalid geometry in document/folder")?,
                ),
            ))
        }
        _ => None,
    })
}
