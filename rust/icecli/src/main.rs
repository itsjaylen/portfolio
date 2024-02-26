mod commands;
mod utils;
mod daemon;

use std::{fs::File, process};
use std::io::Write;


use clap::Parser;
use daemonize::Daemonize;
use log::{ error, info, debug };


use commands::{
    args::{ CommandArgs, EntityCommands },
    developer::handle_developer_command,
    user::handle_user_command,
    video::handle_video_command,
    view::handle_view_command,
};

use crate::utils::{ config::app_config::AppConfig, logger::setup_logger };

fn main() {
    let config = AppConfig::load_config();

    let cli = CommandArgs::parse();

    // Register Command modules here
    match cli.entity_type {
        EntityCommands::User(user) => handle_user_command(user),
        EntityCommands::Video(video) => handle_video_command(video),
        EntityCommands::View(view) => handle_view_command(view),
        EntityCommands::Developer(developer) => {
            if let Err(err) = setup_logger(&config.preferences) {
                eprintln!("Error setting up developer logger: {}", err);
                return;
            }
            if let Err(err) = handle_developer_command(developer) {
                error!("Failed to handle developer command: {:?}", err);
                debug!("Test debug");
            } else {
                info!("Developer command handled successfully");
            }
        }
    }

    // Daemonize the process
    let daemonize = Daemonize::new()
        .pid_file("/tmp/my_daemon.pid")
        .privileged_action(|| {
            // Put your daemon logic here
            println!("Daemon started");

            // Save the PID to a file
            if let Ok(mut file) = File::create("/tmp/my_daemon.pid") {
                if let Err(e) = write!(file, "{}", std::process::id()) {
                    eprintln!("Error writing PID file: {}", e);
                }
            }

            // Your daemon code goes here
            loop {
                // Check for some condition that signals the daemon to exit
                if should_exit() {
                    println!("Exiting daemon gracefully");
                    process::exit(0);
                }

                // Your daemon code continues here
            }
        });


}


// Placeholder function to simulate a condition to exit the daemon
fn should_exit() -> bool {
    // Implement your own logic here
    // For example, return true if a file exists, a specific message is received, etc.
    true
}