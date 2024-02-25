use std::fs::{ self, OpenOptions };

use chrono::Local;
use fern::{ colors::{ Color, ColoredLevelConfig }, Dispatch };

use super::config::Preferences;

pub fn setup_logger(config: &Preferences) -> Result<(), fern::InitError> {
    // Create a logs directory if it doesn't exist
    fs::create_dir_all("logs")?;

    // Generate a timestamp for the log file name
    let timestamp = Local::now().format("%Y-%m-%d");
    let log_file_name = format!("logs/{}.log", timestamp);

    // Print a message before setting up the logger
    println!("Setting up logger for file: {}", log_file_name);

    // Open the file in append mode or create if it doesn't exist
    let log_file = OpenOptions::new().create(true).append(true).open(&log_file_name)?;

    // Set up the logger with colors and include timestamp
    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            let color_config = ColoredLevelConfig::new()
                .error(Color::Red)
                .warn(Color::Yellow)
                .info(Color::Green)
                .debug(Color::Cyan)
                .trace(Color::White);

            out.finish(
                format_args!(
                    "[{timestamp}][{level}][{target}] {message}",
                    timestamp = Local::now().format("%Y-%m-%d %H:%M:%S"),
                    level = color_config.color(record.level()),
                    target = record.target(),
                    message = message
                )
            )
        })
        .chain(log_file)
        .chain(std::io::stderr()); // Chain stderr to print errors to the console

    // Conditionally enable debug logging based on the configuration
    let dispatch = if config.debug_logging {
        dispatch.level(log::LevelFilter::Debug)
    } else {
        dispatch.level(log::LevelFilter::Info)
    };

    // Conditionally enable error logging based on the configuration
    let dispatch = if config.error_logging {
        dispatch.level(log::LevelFilter::Error)
    } else {
        dispatch
    };

    dispatch.apply()?;

    Ok(())
}
