mod commands;
mod utils;
//mod weatherapi;

use clap::Parser;
use log::{ error, info, debug };

use commands::{
    args::{ CommandArgs, EntityCommands },
    developer::handle_developer_command,
    user::handle_user_command,
    video::handle_video_command,
    view::handle_view_command,
    converters::handle_converter_command,
};

use crate::utils::{ config::app_config::AppConfig, logger::setup_logger };

// TODO get a database down
// TODO setup async?
// TODO get together requests

fn main() {
    let config = AppConfig::load_config();

    let cli = CommandArgs::parse();

    let loaded_config = AppConfig::load_config();
    AppConfig::create_config(&loaded_config);

    // Register Command modules here
    match cli.entity_type {
        EntityCommands::User(user) => handle_user_command(user),
        EntityCommands::Video(video) => handle_video_command(video),
        EntityCommands::View(view) => handle_view_command(view),
        EntityCommands::Developer(developer) => {
            if let Err(err) = setup_logger(&config.preferences) {
                eprintln!("Error setting up developer logger: {}", err);
                return;
            }
            if let Err(err) = handle_developer_command(developer) {
                error!("Failed to handle developer command: {:?}", err);
                debug!("Test debug");
            } else {
                info!("Developer command handled successfully");
            }
        }
        EntityCommands::Converter(converter) => {
            if let Err(err) = setup_logger(&config.preferences) {
                eprintln!("Error setting up converter logger: {}", err);
                return;
            }
            if let Err(err) = handle_converter_command(converter) {
                error!("Failed to handle converter command: {:?}", err);
                debug!("Test debug");
            } else {
                info!("converter command handled successfully");
            }
        }
    }
}
