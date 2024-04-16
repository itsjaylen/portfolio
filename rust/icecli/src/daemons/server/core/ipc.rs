use std::io;
use std::time::Instant;

use log::{error, info};
use tokio::{net::TcpListener, runtime::Runtime};

use crate::{
    daemons::server::core::client_handling::accept_connections,
    utils::config::app_config::AppConfig,
};

use super::{client_handling::ClientRequest, commands::status::get_status};

pub enum Command {
    Ping,
    Quit,
    Uptime,
    Testing(Vec<String>),
    Status,
    Unknown,
}

impl Command {
    pub fn execute(&self, uptime_command: &UptimeCommand) -> String {
        match self {
            Command::Ping => self.ping(),
            Command::Quit => self.quit(),
            Command::Uptime => uptime_command.uptime(),
            Command::Testing(args) => self.testing(args),
            Command::Status => get_status(uptime_command), // Call the status function directly
            Command::Unknown => self.unknown(),
        }
    }

    pub fn from_str(command: &str) -> Command {
        let command = command.trim();
        match command.split_whitespace().next() {
            Some("ping") => Command::Ping,
            Some("quit") => Command::Quit,
            Some("uptime") => Command::Uptime,
            Some("status") => Command::Status,
            Some("testing") => {
                let args: Vec<String> = command
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.to_string())
                    .collect();
                Command::Testing(args)
            }
            _ => Command::Unknown,
        }
    }

    fn quit(&self) -> String {
        String::new()
    }

    fn testing(&self, args: &[String]) -> String {
        let args_str = args.join(" ");
        println!("{:?}", args_str);
        format!("Testing command executed with args: {}\n", args_str)
    }

    fn unknown(&self) -> String {
        "Unknown command\n".to_string()
    }
}

#[derive(Clone)]
pub struct UptimeCommand {
    start_time: Instant,
}

impl UptimeCommand {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn uptime(&self) -> String {
        let uptime_duration = self.start_time.elapsed();
        let uptime_secs = uptime_duration.as_secs();
        let uptime_hours = uptime_secs / 3600;
        let uptime_minutes = (uptime_secs % 3600) / 60;
        let uptime_seconds = uptime_secs % 60;

        format!(
            "Uptime: {} hours, {} minutes, {} seconds\n",
            uptime_hours, uptime_minutes, uptime_seconds
        )
    }
}

pub fn parse_request(request_str: &str, server_version: &str) -> Option<ClientRequest> {
    let mut parts = request_str.trim().splitn(3, ':'); // Split at most into 3 parts
    let client_version = parts.next()?.trim().to_string();
    let server_version = server_version.to_string();
    let command = parts.next()?.trim().to_string();
    Some(ClientRequest {
        client_version,
        server_version,
        command,
    })
}

pub fn start_ipc_server() -> io::Result<()> {
    let rt = Runtime::new()?;

    let app_config = AppConfig::load_config();
    let server_config = app_config.server;
    let address = server_config.address;
    let port = server_config.port;

    let server_address = format!("{}:{}", address, port);

    rt.block_on(async {
        match TcpListener::bind(&server_address).await {
            Ok(listener) => {
                info!("Server listening on {}", server_address);
                let uptime_command = UptimeCommand::new();
                accept_connections(listener, uptime_command).await;
            }
            Err(err) => {
                error!("Failed to bind to address {}: {}", server_address, err);
                return Err(err);
            }
        }

        Ok(())
    })
}
