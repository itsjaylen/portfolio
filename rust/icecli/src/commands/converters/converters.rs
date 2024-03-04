use crate::commands::converters::modules::temperature::{
    celsius_to_fahrenheit, fahrenheit_to_celsius, TemperatureUnit,
};

use super::{
    args::{ConverterCommand, ConverterError, ConverterSubcommand},
    modules::temperature::Temperature,
};

#[allow(unused_variables)]
pub fn handle_converter_command(converter: ConverterCommand) -> Result<(), ConverterError> {
    let command = converter.command;
    match command {
        ConverterSubcommand::Temperature(developer) => {
            if let Err(err) = temperature(developer) {
                return Err(ConverterError::TemperatureError("Something".to_string()));
            }
        }
    }

    Ok(())
}

// Commands ran handlers
fn temperature(
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
