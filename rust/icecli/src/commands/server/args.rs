use std::fmt;

use clap::{Args, Subcommand};

use super::modules::logs::ShowLogs;

// Register user subcommand module here
#[derive(Debug, Args)]
pub struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerSubcommand,
}

// Register User subcommands here
#[derive(Debug, Subcommand)]
pub enum ServerSubcommand {
    #[clap(about = "Starts daemon", alias = "startd")]
    StartDaemon,

    #[clap(about = "Starts stops daemon", alias = "stopd")]
    StopDaemon,

    #[clap(about = "Shows daemon logs", alias = "logsd")]
    ShowDaemonLogs(ShowLogs),

    #[clap(about = "Starts server", alias = "start")]
    StartServer,

    #[clap(about = "Stops server", alias = "stop")]
    StopServer,

    #[clap(about = "Shows server logs", alias = "logss")]
    ShowServerLogs(ShowLogs),
}

/// Register Errors here
#[derive(Debug)]
#[allow(dead_code)]
pub enum ServerError {
    DaemonStartError(String),
    DaemonStopError(String),
    DaemonLogsError(String),
    DaemonAlreadyRunning,
    LogFileCreationFailed,

    ServerStartError(String),
    ServerStopError(String),
    ServerDaemonError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::DaemonStartError(msg) => write!(f, "Daemon start error: {}", msg),
            ServerError::DaemonStopError(msg) => write!(f, "Daemon stop error: {}", msg),
            ServerError::DaemonLogsError(msg) => write!(f, "Daemon logs error: {}", msg),
            ServerError::DaemonAlreadyRunning => write!(f, "Daemon is already running."),
            ServerError::LogFileCreationFailed => write!(f, "Failed to create log file."),

            ServerError::ServerStartError(msg) => write!(f, "Server start error: {}", msg),
            ServerError::ServerStopError(msg) => write!(f, "Server stop error: {}", msg),
            ServerError::ServerDaemonError(msg) => write!(f, "Server daemon error: {}", msg),
        }
    }
}
