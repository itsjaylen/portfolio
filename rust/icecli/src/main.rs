mod commands;
mod utils;

use clap::Parser;
use log::{ error, info, debug };

use commands::{
    args::{ CommandArgs, EntityCommands },
    developer::handle_developer_command,
    user::handle_user_command,
    video::handle_video_command,
    view::handle_view_command,
};

use crate::utils::{ config::AppConfig, logger::setup_logger };

fn main() {
    let appconfig = AppConfig::load_config();
    println!("{:?}", appconfig.preferences.theme);

    let cli = CommandArgs::parse();

    // Register Command modules here
    match cli.entity_type {
        EntityCommands::User(user) => handle_user_command(user),
        EntityCommands::Video(video) => handle_video_command(video),
        EntityCommands::View(view) => handle_view_command(view),
        EntityCommands::Developer(developer) => {
            if let Err(err) = setup_logger(&appconfig.preferences) {
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
    }
}
