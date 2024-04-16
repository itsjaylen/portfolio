use clap::{ Parser, Subcommand };

use crate::{
    commands::user::args::UserCommand,
    commands::video::args::VideoCommand,
    commands::view::args::ViewCommand,
    commands::developer::args::DeveloperCommand,
    commands::converters::args::ConverterCommand,
    commands::server::args::ServerCommand,
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
    #[clap(hide = true)]
    User(UserCommand),

    /// Create, update, delete or show videos
    #[clap(hide = true)]
    Video(VideoCommand),

    /// Create or show views
    #[clap(hide = true)]
    View(ViewCommand),

    /// Developer Commands for testing
    #[clap(alias = "dev", alias = "d", hide = false)]
    Developer(DeveloperCommand),

    /// Converter Commands 
    #[clap(alias = "convert", alias = "conv", hide = false )]
    Converter(ConverterCommand),

    /// Server Commands
    #[clap(alias = "ipc", hide = false )]
    Server(ServerCommand)
}
