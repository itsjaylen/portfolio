use std::fs::{ self, OpenOptions };

use chrono::Local;
use dotenvy::dotenv;
use fern::{ colors::{ Color, ColoredLevelConfig }, Dispatch };

use crate::utils::{ constants::HOME_LOG_DIRECTORY, utils::{ get_home_directory, is_debug_mode } };

use super::config::preferences::Preferences;

pub fn setup_logger(config: &Preferences) -> Result<(), fern::InitError> {
    dotenv().ok();
    let debug = is_debug_mode();
    let timestamp = Local::now().format("%Y-%m-%d");

    let logs_directory = if debug {
        // Create the logs directory in the root directory
        let logs_path = format!("logs");
        fs::create_dir_all(&logs_path)?;

        logs_path
    } else {
        // Use home directory for logs if not in debug mode
        let logs_path = format!("{}/{}", get_home_directory(), HOME_LOG_DIRECTORY);
        fs::create_dir_all(&logs_path)?;

        logs_path
    };

    let log_file_name = format!("{}/{}.log", logs_directory, timestamp);


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
