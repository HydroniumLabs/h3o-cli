use geojson::{Feature, JsonObject, JsonValue};
use h3o::{geom::ToGeo, CellIndex};

/// Returns `GeoJSON` features representing the indexes' boundaries.
pub fn boundaries(indexes: &[CellIndex]) -> Vec<Feature> {
    indexes
        .iter()
        .map(|index| {
            let polygon = index.to_geom(true).expect("infaillible");
            let (linestring, _) = polygon.into_inner();
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
