//! Expose [`LatLng::to_cell`]
use anyhow::{Context, Result as AnyResult};
use clap::{ArgGroup, Parser, ValueEnum};
use either::Either;
use h3o::{LatLng, Resolution};

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

    /// Prettify the output (JSON only).
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Run the `latLngToCell` command.
pub fn run(args: &Args) -> AnyResult<()> {
    let indexes = if let (Some(lat), Some(lng)) = (args.lat, args.lng) {
        Either::Left(std::iter::once(
            LatLng::new(lat, lng).context("invalid lat/lng"),
        ))
    } else {
        Either::Right(crate::io::read_coords())
    }
    .map(|input| input.map(|ll| ll.to_cell(args.resolution)));

    match args.format {
        Format::Text => {
            for index in indexes {
                println!("{}", index?);
            }
        }
        Format::Json => {
            let indexes = indexes
                .map(|result| result.map(Into::into))
                .collect::<AnyResult<Vec<crate::json::CellIndex>>>(
            )?;

            crate::json::print(&indexes, args.pretty)?;
        }
    }

    Ok(())
}
