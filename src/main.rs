use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use log::info;
use weatherframe::{ServiceConfig, WeatherFrameService};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ExampleConfig => {
            let example = ServiceConfig::example_config();

            // Note that serializing to a JSON string will expose the API key, here its just an example stub so
            // this is safe.
            let j = serde_json::to_string_pretty(&example)?;
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

            info!("running with service configuration: {:#?}", config);

            let service = WeatherFrameService::new(config);

            // TODO (tff): need some way to interrupt this thing, Ctrl+C handler, etc.
            service.run()?;
        }
    }

    Ok(())
}
