mod commands;

use clap::Parser;

use commands::args::{ CommandArgs, EntityCommands };

use commands::user::handle_user_command;
use commands::video::handle_video_command;
use commands::view::handle_view_command;

fn main() {
    let cli = CommandArgs::parse();


    // Register Command modules here
    match cli.entity_type {
        EntityCommands::User(user) => handle_user_command(user),
        EntityCommands::Video(video) => handle_video_command(video),
        EntityCommands::View(view) => handle_view_command(view),
    }
}
