//! Expose [`LatLng::to_cell`]
use anyhow::{Context, Result as AnyResult};
use clap::{ArgGroup, Parser, ValueEnum};
use h3o::{LatLng, Resolution};
use std::io;

/// Converts from lat/lng coordinates to cell indexes.
///
/// The commands reads lat/lng pairs from stdin until EOF is encountered. For
/// each lat/lng the program outputs to stdout the cell index of the containing
/// cell at the specified resolution.
#[derive(Parser, Debug)]
#[command(group(ArgGroup::new("ll")
    .args(["lat", "lng"])
    .multiple(true)
    .requires_all(["lat", "lng"]))
)]
pub struct Args {
    /// Resolution
    #[arg(short, long)]
    resolution: Resolution,

    /// Latitude in degrees, must be paired with `--lng`.
    #[arg(long)]
    lat: Option<f64>,

    /// Longitude in degrees, mut be paired with `--lat`.
    #[arg(long)]
    lng: Option<f64>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `latLngToCell` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let coords = if let (Some(lat), Some(lng)) = (args.lat, args.lng) {
        vec![LatLng::from_degrees(lat, lng)?]
    } else {
        crate::io::read_coords()?
    };

    let indexes = coords.into_iter().map(|ll| ll.to_cell(args.resolution));
    match args.format {
        Format::Text => {
            for index in indexes {
                println!("{index}");
            }
        }
        Format::Json => {
            let mut stdout = io::stdout().lock();
            let indexes = indexes
                .map(Into::into)
                .collect::<Vec<crate::json::CellIndex>>();
            serde_json::to_writer(&mut stdout, &indexes)
                .context("write JSON to stdout")?;
        }
    }

    Ok(())
}
