use clap::Args;

// Create Video subcommand
#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}
