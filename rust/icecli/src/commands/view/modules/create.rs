use clap::Args;

// Create View subcommand
#[derive(Debug, Args)]
pub struct CreateView {
    /// The id of the user who watched the video
    pub user_id: i32,

    /// The id of the video the user watched
    pub video_id: i32,

    /// The time the user watched the video
    pub watch_start: chrono::NaiveDateTime,

    /// How long the user watched the video for
    pub duration: i32,
}
