use clap::Args;

// User Update subcommand
#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}
