use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use libtatted::{InkyJd79668, Jd79668Config};
use log::info;
use openwx::{GeodeticCoords, WeatherUnits};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the weatherframe service
    Run {
        /// Path to a service configuration file
        #[arg(short, long)]
        config_path: Utf8PathBuf,
    },

    /// Print out an example service configuration and exit
    ExampleConfig,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ExampleConfig => {
            let j = ServiceConfig::example_config()?;
            println!("{j}");
        }
        Commands::Run { config_path } => {
            let level = match cli.debug {
                true => log::Level::Debug,
                false => log::Level::Info,
            };

            // Log messages below the provided level will be filtered out, the RUST_LOG env var is not used here.
            simple_logger::init_with_level(level)?;

            info!("loading config at path: {}", config_path);
            let config =
                ServiceConfig::read_validate_from_path(config_path.clone()).with_context(|| {
                    format!(
                        "failed to load service config file from path '{}'",
                        config_path
                    )
                })?;

            let service = WeatherFrameService::new(config);

            // TODO (tff): need some way to interrupt this thing, Ctrl+C handler, etc.
            service.run()?;
        }
    }

    Ok(())
}

// TODO (tff): probably want to configure the update rate here
/// Service level configuration for weatherframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub coords: GeodeticCoords,
    pub units: WeatherUnits,
    pub api_key: String,
    pub inky: Jd79668Config,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            // Orange County
            coords: GeodeticCoords {
                lat: 33.617,
                lon: -117.831,
            },
            units: WeatherUnits::Imperial,
            api_key: String::from("XXX"),
            inky: Jd79668Config::default(),
        }
    }
}

impl ServiceConfig {
    /// Returns a JSON object representing a default configuration as an example, a fake API key is included
    pub fn example_config() -> Result<String> {
        let j = serde_json::to_string_pretty(&Self::default())?;
        Ok(j)
    }

    pub fn read_validate_from_path(path: Utf8PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read JSON object
        let config: ServiceConfig = serde_json::from_reader(reader)?;

        // Configuration validation steps
        config.coords.validate()?;

        Ok(config)
    }
}

pub struct WeatherFrameService {
    config: ServiceConfig,
}

impl WeatherFrameService {
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }

    pub fn run(&self) -> Result<()> {
        info!("starting WeatherFrame service");

        info!("creating new inky display");
        let mut inky = InkyJd79668::new(Jd79668Config::default()).context(
            "failed to take ownership of hardware peripherals for display communication",
        )?;

        info!("initializing inky display");
        inky.initialize()?;

        loop {
            info!(
                "querying OpenWeather for updated weather at position: lat: {:.3}, lon: {:.3}",
                self.config.coords.lat, self.config.coords.lon
            );
            let response = openwx::open_weather_request(
                self.config.coords,
                WeatherUnits::Imperial,
                self.config.api_key.clone(),
            )?;

            // TODO: use the response to a render a new image for the display

            info!("new OpenWeather response: {:#?}", response);

            // 20 minutes
            thread::sleep(Duration::from_secs(1200));
        }
    }
}
