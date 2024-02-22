use clap::{ Args, Subcommand };

use super::modules::{ create::CreateVideo, delete::DeleteEntity, update::UpdateVideo };

// Register video module sub command here
#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

// Register Video subcommands here
#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}
