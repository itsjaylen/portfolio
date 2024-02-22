use clap::Args;

// User create subcommand
#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,

    /// The passowrd of the user
    pub password: String,
}
