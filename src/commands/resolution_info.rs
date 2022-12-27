//! Expose resolution information.

use anyhow::{Context, Result as AnyResult};
use clap::{Parser, ValueEnum};
use h3o::Resolution;
use serde::Serialize;
use std::{fmt, io};

/// Print cell statistics per resolution.
#[derive(Parser, Debug)]
pub struct Args {
    /// h3o index.
    #[arg(short, long)]
    resolution: Option<Resolution>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `indexDecode` command.
pub fn run(args: &Args) -> AnyResult<()> {
    if let Some(resolution) = args.resolution {
        let info = ResolutionInfo::from(resolution);
        match args.format {
            Format::Text => println!("{info}"),
            Format::Json => {
                serde_json::to_writer(&mut io::stdout(), &info)
                    .context("write JSON to stdout")?;
            }
        }
        return Ok(());
    }

    match args.format {
        Format::Text => {
            println!("╔═{c:═>10}═╦═{c:═>15}═╦═{c:═>20}═╦═{c:═>14}═╗", c = '═');
            println!(
                "║ {:<10} ║ {:<15} ║ {:<20} ║ {:<14} ║",
                "Resolution", "Cell count", "Hexagon area", "Edge length"
            );
            println!("╠═{c:═>10}═╬═{c:═>15}═╬═{c:═>20}═╬═{c:═>14}═╣", c = '═');
            for resolution in
                Resolution::range(Resolution::Zero, Resolution::Fifteen)
            {
                let info = ResolutionInfo::from(resolution);
                let (area, area_unit, length, length_unit) = if info.resolution
                    < Resolution::Eight
                {
                    (info.hexagon_area_km2, "km2", info.edge_length_km, "km")
                } else {
                    (
                        info.hexagon_area_km2 * 1e6,
                        "m2",
                        info.edge_length_km * 1e3,
                        "m",
                    )
                };
                println!(
                    "║ {:>10} ║ {:>15} ║ {:16.3} {:<3} ║ {:11.3} {:<2} ║",
                    u8::from(info.resolution),
                    info.cell_count,
                    area,
                    area_unit,
                    length,
                    length_unit
                );
            }
            println!("╚═{c:═>10}═╩═{c:═>15}═╩═{c:═>20}═╩═{c:═>14}═╝", c = '═');
        }
        Format::Json => {
            let infos =
                Resolution::range(Resolution::Zero, Resolution::Fifteen)
                    .map(Into::into)
                    .collect::<Vec<ResolutionInfo>>();
            serde_json::to_writer(&mut io::stdout(), &infos)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------

#[derive(Serialize)]
struct ResolutionInfo {
    resolution: Resolution,
    cell_count: u64,
    hexagon_area_km2: f64,
    edge_length_km: f64,
}

impl From<Resolution> for ResolutionInfo {
    fn from(value: Resolution) -> Self {
        Self {
            resolution: value,
            cell_count: value.cell_count(),
            hexagon_area_km2: value.area_km2(),
            edge_length_km: value.edge_length_km(),
        }
    }
}

impl fmt::Display for ResolutionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "resolution:   {}", self.resolution)?;
        writeln!(f, "cell count:   {}", self.cell_count)?;
        if self.resolution < Resolution::Eight {
            writeln!(f, "hexagon area: {} km2", self.hexagon_area_km2)?;
            write!(f, "edge length:  {} km", self.edge_length_km)
        } else {
            writeln!(f, "hexagon area: {} m2", self.hexagon_area_km2 * 1e6)?;
            write!(f, "edge length:  {} m", self.edge_length_km * 1e3)
        }
    }
}
