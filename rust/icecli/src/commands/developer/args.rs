use clap::{ Args, Subcommand };

use super::modules::tests::Test;

// Register user subcommand module here
#[derive(Debug, Args)]
pub struct DeveloperCommand {
    #[clap(subcommand)]
    pub command: DeveloperSubcommand,
}

// Register User subcommands here
#[derive(Debug, Subcommand)]
pub enum DeveloperSubcommand {
    /// Run test command
    Test(Test),
}
