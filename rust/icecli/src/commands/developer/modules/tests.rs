use clap::Args;

// Developer test subcommand
#[derive(Debug, Args)]
pub struct Test {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,

    /// The passowrd of the user
    pub password: String,
}
