mod commands;
mod utils;

use clap::Parser;


use utils::config::AppConfig;

use commands::{
    args::{ CommandArgs, EntityCommands },
    developer::handle_developer_command,
    user::handle_user_command,
    video::handle_video_command,
    view::handle_view_command,
};


fn main() {

    AppConfig::load_config();


    let cli = CommandArgs::parse();

    // Register Command modules here
    match cli.entity_type {
        EntityCommands::User(user) => handle_user_command(user),
        EntityCommands::Video(video) => handle_video_command(video),
        EntityCommands::View(view) => handle_view_command(view),
        EntityCommands::Developer(developer) => handle_developer_command(developer),
    }
}
