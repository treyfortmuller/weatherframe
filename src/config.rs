use anyhow::Result;
use camino::Utf8PathBuf;
use libtatted::Jd79668Config;
use openwx::{GeodeticCoords, WeatherUnits};
use redact::{Secret, expose_secret};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

/// Service level configuration for weatherframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub coords: GeodeticCoords,
    pub units: WeatherUnits,
    #[serde(serialize_with = "expose_secret")]
    pub api_key: Secret<String>,
    pub refresh_interval: Duration,
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
            api_key: Secret::new(String::from("XXX")),
            refresh_interval: Duration::from_secs(1200), // 20 minutes
            inky: Jd79668Config::default(),
        }
    }
}

impl ServiceConfig {
    /// Returns a JSON object representing a default configuration as an example, a fake API key is included
    pub fn example_config() -> Self {
        Self::default()
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
