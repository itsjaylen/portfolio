// Import specific ConversionResult types
use crate::commands::converters::modules::distance::ConversionResult as DistanceConversionResult;
use crate::commands::converters::modules::speed::ConversionResult as SpeedConversionResult;
use crate::commands::converters::modules::{
    distance::DistanceUnit,
    speed::SpeedUnit,
    temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius, TemperatureUnit},
};

use super::{
    args::{ConverterCommand, ConverterError, ConverterSubcommand},
    modules::{distance::Distance, speed::Speed, temperature::Temperature},
};


#[allow(unused_variables)]
pub fn handle_converter_command(converter: ConverterCommand) -> Result<(), ConverterError> {
    let command = converter.command;
    match command {
        ConverterSubcommand::Temperature(converter) => {
            if let Err(err) = temperature_converter(converter) {
                let error_message = err.to_string();
                return Err(ConverterError::TemperatureError(error_message));
            }
        }

        ConverterSubcommand::Speed(converter) => {
            if let Err(err) = speed_converter(converter) {
                return Err(ConverterError::SpeedError("Something".to_string()));
            }
        }

        ConverterSubcommand::Distance(converter) => {
            if let Err(err) = distance_converter(converter) {
                let error_message = err.to_string();
                return Err(ConverterError::DistanceError(error_message))
            }
        }
    }

    Ok(())
}

// Commands ran handlers
// TODO Move this into its own file
fn temperature_converter(
    converter: Temperature,
) -> Result<(), crate::commands::converters::converters::ConverterError> {
    match converter.unit {
        TemperatureUnit::C | TemperatureUnit::Celsius => {
            let converted_temperature = celsius_to_fahrenheit(converter.temperature);
            println!("Converted temperature: {}°F", converted_temperature);
        }
        TemperatureUnit::F | TemperatureUnit::Fahrenheit => {
            let converted_temperature = fahrenheit_to_celsius(converter.temperature);
            println!("Converted temperature: {}°C", converted_temperature);
        }
    }

    Ok(())
}

fn speed_converter(
    converter: Speed,
) -> Result<(), crate::commands::converters::converters::ConverterError> {

    let converted_unit = SpeedUnit::convert_units(
        converter.speed,
        &converter.output_unit.to_string(),
        &converter.input_unit,
        converter.return_json,
        converter.round_values,
    );

    if converter.return_json.unwrap_or(false) {
        match converted_unit {
            Ok(json_str) => {
                let result: Result<SpeedConversionResult, serde_json::Error> =
                    SpeedUnit::deserialize_conversion_result(&json_str);

                match result {
                    Ok(converted_result) => {
                        println!("{:?}", converted_result);
                    }
                    Err(e) => {
                        return Err(ConverterError::SpeedError(format!(
                            "Conversion error: Error deserializing JSON: {}",
                            e
                        )));
                    }
                }
            }
            Err(e) => {
                return Err(ConverterError::SpeedError(format!(
                    "Conversion error: Error converting to JSON: {}",
                    e
                )));
            }
        }
    } else {
        match converted_unit {
            Ok(value) => {
                println!("{}", value);
            }
            Err(e) => {
                return Err(ConverterError::SpeedError(format!(
                    "Conversion error: {}",
                    e
                )));
            }
        }
    }

    Ok(())
}

fn distance_converter(
    converter: Distance,
) -> Result<(), crate::commands::converters::converters::ConverterError> {
    let converted_unit = DistanceUnit::convert_units(
        converter.distance,
        &converter.output_unit.to_string(),
        &converter.input_unit,
        converter.return_json,
        converter.round_values,
    );
   
    if converter.return_json.unwrap_or(false) {
        match converted_unit {
            Ok(json_str) => {
                let result: Result<DistanceConversionResult, serde_json::Error> =
                    DistanceUnit::deserialize_conversion_result(&json_str);

                match result {
                    Ok(converted_result) => {
                        println!("{:?}", converted_result);
                    }
                    Err(e) => {
                        return Err(ConverterError::DistanceError(format!(
                            "Conversion error: Error deserializing JSON: {}",
                            e
                        )));
                    }
                }
            }
            Err(e) => {
                return Err(ConverterError::DistanceError(format!(
                    "Conversion error: Error converting to JSON: {}",
                    e
                )));
            }
        }
    } else {
        match converted_unit {
            Ok(value) => {
                println!("{}", value);
            }
            Err(e) => {
                return Err(ConverterError::DistanceError(format!(
                    "Conversion error: {}",
                    e
                )));
            }
        }
    }

    Ok(())
}
