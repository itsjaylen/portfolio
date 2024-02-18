mod user;

use user::args::EntityType;
use user::args::RustflixArgs;
use clap::Parser;
use user::ops::user_ops::handle_user_command;
use user::ops::video_ops::handle_video_command;
use user::ops::view_ops::handle_view_command;

fn main() {
    let args = RustflixArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
        EntityType::Video(video) => handle_video_command(video),
        EntityType::View(view) => handle_view_command(view),
    };
}
