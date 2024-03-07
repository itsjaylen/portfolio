use clap::{Args, Subcommand};

use super::modules::{speed::Speed, temperature::Temperature};

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
    
    #[clap(about = "Convert speeds",)]
    Speed(Speed),
}

/// Register Errors here
#[derive(Debug)]
#[allow(dead_code)]
pub enum ConverterError {
    TemperatureError(String),
    SpeedError(String),
}
