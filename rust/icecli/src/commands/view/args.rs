use clap::{ Args, Subcommand };

use super::modules::create::CreateView;

// Register view module subcommand here
#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

// Register view subcommands
#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// Create a new view
    Create(CreateView),

    /// Show all views with id numbers for users and videos
    Show,

    /// Show all views with names for users and videos
    ShowPretty,
}


