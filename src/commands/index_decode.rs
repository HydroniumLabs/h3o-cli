//! Expose index's components

use crate::index::Index;
use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::{BaseCell, Direction, Edge, IndexMode, Resolution, Vertex};
use serde::Serialize;
use std::io;

/// Decode h3o indexes into components
#[derive(Parser, Debug)]
pub struct Args {
    /// h3o index.
    #[arg(short, long)]
    index: Option<Index>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Compact)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Compact,
    Pretty,
    Json,
}

/// Run the `indexDecode` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = if let Some(index) = args.index {
        vec![index]
    } else {
        crate::io::read_indexes().context("read Index from stdin")?
    };

    let components = indexes
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Components>>();
    match args.format {
        Format::Compact => components_to_compact(&components),
        Format::Pretty => components_to_pretty(&components),
        Format::Json => components_to_json(&components)?,
    }

    Ok(())
}

fn components_to_compact(components: &[Components]) {
    for component in components {
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
}

fn components_to_pretty(components: &[Components]) {
    for component in components {
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
}

fn components_to_json(components: &[Components]) -> AnyResult<()> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, components)
        .context("write JSON to stdout")
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
