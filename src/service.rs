use crate::ServiceConfig;
use anyhow::{Context, Result};
use libtatted::{InkyJd79668, Jd79668Config};
use log::{info, warn};
use openwx::WeatherUnits;
use std::thread;
use std::time::{Duration, Instant};

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

        // Main program loop
        let interval = Duration::from_secs(1); // TODO (tff): pull in the desired interval from the config
        let mut next_refresh = std::time::Instant::now() + interval;

        loop {
            info!(
                "querying OpenWeather for updated weather at position: lat: {:.3}, lon: {:.3}",
                self.config.coords.lat, self.config.coords.lon
            );
            let response = openwx::open_weather_request(
                self.config.coords,
                WeatherUnits::Imperial,
                self.config.api_key.expose_secret().clone(),
            )?;

            // TODO: use the response to a render a new image for the display

            info!("new OpenWeather response: {:#?}", response);

            // Calculate the time to sleep until the next scheduled execution
            let now = Instant::now();
            if now < next_refresh {
                thread::sleep(next_refresh - now);
            } else {
                warn!("weather polling + display update took longer than the refresh interval");
            }

            // Update the next scheduled execution time
            next_refresh += interval;
        }
    }
}
