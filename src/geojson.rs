use geo_types::{LineString, coord};
use geojson::{Feature, JsonObject, JsonValue};
use h3o::{CellIndex, LatLng};

/// Returns `GeoJSON` features representing the indexes' boundaries.
pub fn boundaries(indexes: &[CellIndex]) -> Vec<Feature> {
    indexes
        .iter()
        .map(|index| {
            let linestring: LineString = index.boundary().into();
            let geometry = geojson::Geometry::new((&linestring).into());
            let mut properties = JsonObject::new();
            properties
                .insert("name".to_owned(), JsonValue::from(index.to_string()));
            Feature {
                bbox: None,
                geometry: Some(geometry),
                id: Some(geojson::feature::Id::Number(
                    u64::from(*index).into(),
                )),
                properties: Some(properties),
                foreign_members: None,
            }
        })
        .collect::<Vec<_>>()
}

/// Returns `GeoJSON` features representing the indexes' centers.
pub fn centers(indexes: &[CellIndex]) -> Vec<Feature> {
    indexes
        .iter()
        .copied()
        .map(|index| {
            let ll = LatLng::from(index);
            let center = geo_types::Point(coord! {x: ll.lng(), y: ll.lat()});
            let geometry = geojson::Geometry::new((&center).into());
            let mut properties = JsonObject::new();
            properties
                .insert("name".to_owned(), JsonValue::from(index.to_string()));
            Feature {
                bbox: None,
                geometry: Some(geometry),
                id: Some(geojson::feature::Id::Number(u64::from(index).into())),
                properties: Some(properties),
                foreign_members: None,
            }
        })
        .collect::<Vec<_>>()
}
