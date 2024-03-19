use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub converted_value: f64,
    pub output_unit: String,
}

pub trait Unit {
    fn unit_conversions(unit: &str) -> f64;
    fn convert_units(
        value: f64,
        input_unit: &str,
        output_unit: &str,
        return_json: Option<bool>,
        round_values: Option<bool>,
    ) -> Result<String, SerdeJsonError> {
        let factor_from = Self::unit_conversions(input_unit);
        let factor_to = Self::unit_conversions(output_unit);
        let mut converted_value = value * (factor_from / factor_to);

        // Round the value if round_values is true
        if round_values.unwrap_or(false) {
            converted_value = converted_value.round();
        }

        // Generate JSON or return the value
        if return_json.unwrap_or(false) {
            let result = ConversionResult {
                converted_value,
                output_unit: output_unit.to_string(),
            };
            serde_json::to_string_pretty(&result)
        } else {
            Ok(converted_value.to_string())
        }
    }
}
