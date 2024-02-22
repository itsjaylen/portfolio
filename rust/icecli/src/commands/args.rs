use clap::{ Parser, Subcommand };

use crate::{
    commands::user::args::UserCommand,
    commands::video::args::VideoCommand,
    commands::view::args::ViewCommand,
};

// Register the main entity here that handles all the args.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CommandArgs {
    #[clap(subcommand)]
    pub entity_type: EntityCommands,
}

// Register Commands here
#[derive(Debug, Subcommand)]
pub enum EntityCommands {
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show videos
    Video(VideoCommand),

    /// Create or show views
    View(ViewCommand),
}
