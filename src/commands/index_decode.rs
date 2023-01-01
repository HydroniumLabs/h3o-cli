//! Expose index's components

use crate::index::Index;
use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use either::Either;
use h3o::{BaseCell, Direction, Edge, IndexMode, Resolution, Vertex};
use serde::Serialize;

/// Decode h3o indexes into components
#[derive(Parser, Debug)]
pub struct Args {
    /// h3o index.
    #[arg(short, long)]
    index: Option<Index>,

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
}

/// Run the `indexDecode` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let components = args
        .index
        .map_or_else(
            || Either::Right(crate::io::read_indexes()),
            |index| Either::Left(std::iter::once(Ok(index))),
        )
        .map(|input| input.map(Components::from));

    match args.format {
        Format::Text => {
            if args.pretty {
                components_to_pretty(components)
            } else {
                components_to_compact(components)
            }
        }
        Format::Json => components_to_json(components, args.pretty),
    }
    .context("indexDecode")?;

    Ok(())
}

fn components_to_compact(
    components: impl IntoIterator<Item = AnyResult<Components>>,
) -> AnyResult<()> {
    for component in components {
        let component = component?;
        let mode = u8::from(component.mode);
        let resolution = u8::from(component.resolution);
        let base_cell = u8::from(component.base_cell);
        let directions = component
            .directions
            .iter()
            .map(ToString::to_string)
            .collect::<String>();

        component.custom.as_ref().map_or_else(
            || {
                println!("{mode}:{resolution}:{base_cell}:{directions}");
            },
            |field| {
                println!(
                    "{mode}:{}:{resolution}:{base_cell}:{directions}",
                    u8::from(*field),
                );
            },
        );
    }

    Ok(())
}

fn components_to_pretty(
    components: impl IntoIterator<Item = AnyResult<Components>>,
) -> AnyResult<()> {
    for component in components {
        let component = component?;

        println!("╔════════════╗");
        println!("║ h3o Index  ║ {}", component.index);
        println!("╠════════════╣");
        println!(
            "║ Mode       ║ {} ({})",
            component.mode,
            u8::from(component.mode)
        );
        println!("║ Resolution ║ {}", component.resolution);
        match component.custom {
            Some(CustomField::Edge(edge)) => println!("║ Edge       ║ {edge}"),
            Some(CustomField::Vertex(vertex)) => {
                println!("║ Vertex     ║ {vertex}");
            }
            _ => (),
        }
        println!("║ Base Cell  ║ {}", component.base_cell);
        for (i, direction) in component.directions.iter().enumerate() {
            println!("║ Child {:>2}   ║ {direction} ({direction:?})", i + 1);
        }
        println!("╚════════════╝");
    }

    Ok(())
}

fn components_to_json(
    components: impl IntoIterator<Item = AnyResult<Components>>,
    pretty: bool,
) -> AnyResult<()> {
    let components = components.into_iter().collect::<AnyResult<Vec<_>>>()?;

    crate::json::print(&components, pretty)
}

// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Components {
    index: String,
    mode: IndexMode,
    #[serde(flatten)]
    custom: Option<CustomField>,
    resolution: Resolution,
    base_cell: BaseCell,
    directions: Vec<Direction>,
}

impl From<Index> for Components {
    fn from(value: Index) -> Self {
        let (index, mode, custom, cell) = match value {
            Index::Cell(index) => {
                (index.to_string(), IndexMode::Cell, None, index)
            }
            Index::DirectedEdge(index) => (
                index.to_string(),
                IndexMode::DirectedEdge,
                Some(CustomField::Edge(index.edge())),
                index.origin(),
            ),
            Index::Vertex(index) => (
                index.to_string(),
                IndexMode::Vertex,
                Some(CustomField::Vertex(index.vertex())),
                index.owner(),
            ),
        };

        Self {
            index,
            mode,
            custom,
            resolution: cell.resolution(),
            base_cell: cell.base_cell(),
            directions: Resolution::range(Resolution::One, cell.resolution())
                .map(|resolution| {
                    cell.direction_at(resolution).expect("direction")
                })
                .collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CustomField {
    Edge(Edge),
    Vertex(Vertex),
}

impl From<CustomField> for u8 {
    fn from(value: CustomField) -> Self {
        match value {
            CustomField::Edge(edge) => edge.into(),
            CustomField::Vertex(vertex) => vertex.into(),
        }
    }
}
