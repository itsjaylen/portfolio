use clap::{Args, ValueEnum, ValueHint};

// Temperature
#[derive(Debug, Args, Clone)]
#[clap(about = "case")]
pub struct Temperature {
    /// Temperature value
    #[arg(
        short,
        long,
        ignore_case(true),
        value_hint(ValueHint::Unknown),
        help("The temperature your converting")
    )]
    pub temperature: f64,

    #[arg(
        short,
        long,
        ignore_case(true),
        help("Unit fahrenheit or celsius"),
        default_value = "Celsius"
    )] 
    pub unit: TemperatureUnit,
}

#[derive(Debug, ValueEnum, Clone)]
#[clap(rename_all = "kebab-case")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    C,
    F,
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
