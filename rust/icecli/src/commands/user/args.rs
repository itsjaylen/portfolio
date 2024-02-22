use clap::{ Args, Subcommand };

use super::modules::{ create::CreateUser, delete::DeleteEntity, update::UpdateUser };

// Register user subcommand module here
#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

// Register User subcommands here
#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}
