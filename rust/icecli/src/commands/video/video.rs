use super::{
    args::{ VideoCommand, VideoSubcommand },
    modules::{ create::CreateVideo, delete::DeleteEntity, update::UpdateVideo },
};

// Handles all video subcommands
pub fn handle_video_command(video: VideoCommand) {
    let command = video.command;
    match command {
        VideoSubcommand::Create(video) => {
            create_video(video);
        }
        VideoSubcommand::Update(video) => {
            update_video(video);
        }
        VideoSubcommand::Delete(delete_entity) => {
            delete_video(delete_entity);
        }
        VideoSubcommand::Show => {
            show_videos();
        }
    }
}

// Commands ran handlers

pub fn create_video(video: CreateVideo) {
    println!("Creating video: {:?}", video);
}

pub fn update_video(video: UpdateVideo) {
    println!("Updating video: {:?}", video);
}

pub fn delete_video(video: DeleteEntity) {
    println!("Deleting video: {:?}", video);
}

pub fn show_videos() {
    println!("Showing videos");
}
