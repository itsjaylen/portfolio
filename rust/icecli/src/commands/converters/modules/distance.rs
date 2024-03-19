use clap::{Args, ValueEnum, ValueHint};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::commands::converters::units::Unit;

#[derive(Debug, Clone, ValueEnum)]
pub enum DistanceUnit {
    Meters,
    Kilometers,
    Inches,
    Feet,
    Miles,
    Millimeters,
    Centimeters,
}

/// For if the unit is converted to json
#[derive(Serialize, Deserialize, Debug)]
pub struct ConversionResult {
    pub converted_distance: f64,
    pub output_unit: String,
}

/// Format the unit type so it's the default
impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistanceUnit::Meters => write!(f, "Meters"),
            DistanceUnit::Kilometers => write!(f, "Kilometers"),
            DistanceUnit::Inches => write!(f, "Inches"),
            DistanceUnit::Feet => write!(f, "Feet"),
            DistanceUnit::Miles => write!(f, "Miles"),
            DistanceUnit::Millimeters => write!(f, "Millimeters"),
            DistanceUnit::Centimeters => write!(f, "Centimeters"),
        }
    }
}

impl DistanceUnit {
    /// Cleans the json object
    pub fn deserialize_conversion_result(
        json_str: &str,
    ) -> Result<ConversionResult, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

impl Unit for DistanceUnit {
    /// Based on 1 mile
    fn unit_conversions(unit: &str) -> f64 {
        match unit {
            "Meters" => 1609.34,
            "Kilometers" => 1.60934,
            "Inches" => 63360.0,
            "Feet" => 5280.0,
            "Miles" => 1.0,
            "Millimeters" => 1609344.0,
            "Centimeters" => 160934.4,
            _ => {
                println!("Unsupported unit: {}", unit);
                1.0 // Default to 1.0 for unsupported units
            }
        }
    }
}

#[derive(Debug, Clone, Args)]
#[clap(after_help = "\x1b[1mEXAMPLES:\x1b[0m\n
    - \x1b[1mConvert 50 miles to km use:\x1b[0m 
        --distance 50 --input-unit miles --output-unit kilometers 
")]
pub struct Distance {
    #[arg(
        short,
        long,
        ignore_case(true),
        value_hint(ValueHint::Unknown),
        required(true),
        help = "The distance value"
    )]
    pub distance: f64,

    /// The distance unit being inputted
    #[arg(
        short = 'i',
        long = "input-unit",
        ignore_case(true),
        default_value("Miles"),
        help = "Input unit of speed (default: Miles)"
    )]
    pub input_unit: DistanceUnit,

    /// The distance unit the value is being converted to
    #[arg(
        short = 'o',
        long = "output-unit",
        ignore_case(true),
        default_value("Kilometers"),
        help = "Output unit of speed (default: Kilometers)"
    )]
    pub output_unit: DistanceUnit,

    /// Arg to check if you want to return as json
    #[arg(
        short = 'j',
        long = "return-json",
        ignore_case(true),
        default_value("false"),
        help = "If to return as a json"
    )]
    pub return_json: Option<bool>,

    /// Arg to check if you want to round the values
    #[arg(
        short = 'r',
        long = "round_values",
        ignore_case(true),
        default_value("false"),
        help = "If to round all values"
    )]
    pub round_values: Option<bool>,
}
