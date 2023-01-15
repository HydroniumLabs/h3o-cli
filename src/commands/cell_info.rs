//! Expose cell index information.

use anyhow::Result as AnyResult;
use clap::{Parser, ValueEnum};
use h3o::{BaseCell, CellIndex, Face, LatLng, Resolution};
use serde::Serialize;
use std::fmt;

/// Print a bunch of info on a cell index.
#[derive(Parser, Debug)]
pub struct Args {
    /// Cell index.
    #[arg(short, long)]
    index: Option<CellIndex>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Prettify the output (JSON only).
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `cellInfo` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = crate::utils::get_cell_indexes(args.index);
    let infos = indexes.map(|input| input.map(CellInfo::from));

    match args.format {
        Format::Text => {
            for info in infos {
                println!("{}", info?);
            }
        }
        Format::Json => {
            let infos = infos.collect::<AnyResult<Vec<_>>>()?;
            crate::json::print(&infos, args.pretty)?;
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct CellInfo {
    index: String,
    base_cell: BaseCell,
    resolution: Resolution,
    center: (f64, f64),
    area: f64,
    edge_length: f64,
    parent: Option<String>,
    children: Vec<String>,
    neighbors: Vec<String>,
    vertexes: Vec<String>,
    edges: Vec<String>,
    faces: Vec<Face>,
    is_pentagon: bool,
    is_class3: bool,
}

impl From<CellIndex> for CellInfo {
    fn from(value: CellIndex) -> Self {
        let edges = value.edges().collect::<Vec<_>>();
        let ll = LatLng::from(value);

        Self {
            index: value.to_string(),
            base_cell: value.base_cell(),
            resolution: value.resolution(),
            center: (ll.lat(), ll.lng()),
            area: value.area_km2(),
            edge_length: edges[0].length_km(),
            parent: value
                .resolution()
                .pred()
                .map(|res| value.parent(res).expect("parent").to_string()),
            children: value
                .resolution()
                .succ()
                .map(|res| {
                    value.children(res).map(|cell| cell.to_string()).collect()
                })
                .unwrap_or_default(),
            neighbors: value
                .grid_disk_safe(1)
                .map(|cell| cell.to_string())
                .collect(),
            vertexes: value
                .vertexes()
                .map(|vertex| vertex.to_string())
                .collect(),
            edges: edges.iter().map(ToString::to_string).collect(),
            faces: value.icosahedron_faces().iter().collect(),
            is_pentagon: value.is_pentagon(),
            is_class3: value.resolution().is_class3(),
        }
    }
}

impl fmt::Display for CellInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent = self
            .parent
            .as_ref()
            .map_or_else(|| "N/A".to_owned(), ToString::to_string);
        let faces = self
            .faces
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "index:            {}", self.index)?;
        writeln!(f, "base cell:        {}", self.base_cell)?;
        writeln!(f, "resolution:       {}", self.resolution)?;
        writeln!(
            f,
            "center:           {:.9} {:.9}",
            self.center.0, self.center.1
        )?;
        if self.resolution < Resolution::Eight {
            writeln!(f, "area (km2):       {:.3}", self.area)?;
            writeln!(f, "edge length (km): {:.3}", self.edge_length)?;
        } else {
            writeln!(f, "area (m2):        {:.3}", self.area * 1e6)?;
            writeln!(f, "edge length (m):  {:.3}", self.edge_length * 1e3)?;
        }
        writeln!(f, "parent:           {parent}")?;
        writeln!(f, "children:         [{}]", self.children.join(", "))?;
        writeln!(f, "neighbors:        [{}]", self.neighbors.join(", "))?;
        writeln!(f, "vertexes:         [{}]", self.vertexes.join(", "))?;
        writeln!(f, "edges:            [{}]", self.edges.join(", "))?;
        writeln!(f, "faces:            [{faces}]")?;
        writeln!(f, "isPentagon:       {}", self.is_pentagon)?;
        write!(f, "isClassIII:       {}", self.is_class3)
    }
}
