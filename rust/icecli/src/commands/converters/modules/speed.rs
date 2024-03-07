use clap::{Args, ValueEnum, ValueHint};
use serde::{Deserialize, Serialize};
use std::fmt;

// The types of speed units
#[derive(Debug, Clone, ValueEnum)]
pub enum SpeedUnit {
    Mph,
    Kph,
    Knots,
    MetersPerSecond,
    FeetPerSecond,
}


/// For if the unit is converted to json
#[derive(Serialize, Deserialize, Debug)]
pub struct ConversionResult {
    pub converted_speed: f64,
    pub output_unit: String,
}

/// Format the unit type so it's the default
impl fmt::Display for SpeedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpeedUnit::Mph => write!(f, "mph"),
            SpeedUnit::Kph => write!(f, "kph"),
            SpeedUnit::Knots => write!(f, "knots"),
            SpeedUnit::MetersPerSecond => write!(f, "m/s"),
            SpeedUnit::FeetPerSecond => write!(f, "fps"),
        }
    }
}

impl SpeedUnit {
    /// Convert the given speed value from one unit to another with given args
    pub fn convert_units(
        speed: f64,
        input_unit: &str,
        output_unit: &SpeedUnit,
        return_json: Option<bool>,
        round_values: Option<bool>,
    ) -> Result<String, serde_json::Error> {
        // Convert the speed to the specified unit
        let factor_from = SpeedUnit::unit_conversions(input_unit);
        let factor_to = SpeedUnit::unit_conversions(output_unit.to_string().as_str());
        let mut converted_speed = speed * (factor_from / factor_to);

        // Round the value if round_values is true
        if round_values.unwrap_or(false) {
            converted_speed = converted_speed.round();
        }

        // Generate JSON or return the value
        if return_json.unwrap_or(false) {
            let result = ConversionResult {
                converted_speed: converted_speed,
                output_unit: output_unit.to_string(),
            };
            serde_json::to_string_pretty(&result)
        } else {
            Ok(converted_speed.to_string())
        }
    }

    /// Cleans the json object
    pub fn deserialize_conversion_result(json_str: &str) -> Result<ConversionResult, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// This is now a public static method within the SpeedUnit enum
    pub fn unit_conversions(unit: &str) -> f64 {
        match unit {
            "mph" => 1.0,
            "kph" => 1.60934,
            "knots" => 0.868976,
            "m/s" => 0.44704,
            "fps" => 1.46667,
            _ => {
                println!("Unsupported unit: {}", unit);
                1.0 // Default to 1.0 for unsupported units
            }
        }
    }
}

/// The speed unit args
#[derive(Debug, Clone, Args)]
#[clap(after_help = "\x1b[1mEXAMPLES:\x1b[0m
    - \x1b[1mConvert 50 kph to mph use:\x1b[0m
      --speed 50 --input-unit kph --output-unit mph
    - \x1b[1mConvert 100 meters per second to knots:\x1b[0m
      --speed 100 --input-unit m/s --output-unit knots
      \x1b[4m\x1b[1mNote:\x1b[0m\x1b[0m You can customize the command with different speed values and units.")]
pub struct Speed {
    /// The speed unit being converted
    #[arg(
        short,
        long,
        ignore_case(true),
        value_hint(ValueHint::Unknown),
        required(true),
        help = "The speed value"
    )]
    pub speed: f64,

    /// The speed unit being inputted
    #[arg(
        short = 'i',
        long = "input-unit",
        ignore_case(true),
        default_value("mph"),
        help = "Input unit of speed (default: mph)"
    )]
    pub input_unit: SpeedUnit,

    /// The speed unit the value is being converted to
    #[arg(
        short = 'o',
        long = "output-unit",
        ignore_case(true),
        default_value("kph"),
        help = "Output unit of speed (default: kph)"
    )]
    pub output_unit: SpeedUnit,

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

