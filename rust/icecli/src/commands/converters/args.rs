use std::fmt;

use clap::{Args, Subcommand};

use super::modules::{distance::Distance, speed::Speed, temperature::Temperature};

// Register converter subcommand module here
#[derive(Debug, Args)]
pub struct ConverterCommand {
    #[clap(subcommand)]
    pub command: ConverterSubcommand,
}

// Register converter subcommands here
#[derive(Debug, Subcommand)]
pub enum ConverterSubcommand {
    #[clap(about = "Convert temperatures", alias = "temp")]
    Temperature(Temperature),

    #[clap(about = "Convert speeds")]
    Speed(Speed),

    #[clap(about = "Convert Distances")]
    Distance(Distance)
}

/// Register Errors here
#[derive(Debug)]
#[allow(dead_code)]
pub enum ConverterError {
    TemperatureError(String),
    SpeedError(String),
    DistanceError(String),
}

impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConverterError::TemperatureError(message) => {
                write!(f, "TemperatureError: {}", message)
            }

            ConverterError::SpeedError(message) => {
                write!(f, "SpeedError: {}", message)
            }

            ConverterError::DistanceError(message) => {
                write!(f, "DistanceError: {}", message)
            }
        }
    }
}
