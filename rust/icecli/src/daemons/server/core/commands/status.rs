use serde::{Serialize, Deserialize};

use crate::daemons::server::core::ipc::UptimeCommand;

#[derive(Serialize, Deserialize)]
pub struct StatusResult {
    pub ping: String,
    pub uptime: String,
}

impl StatusResult {
    pub fn new(ping: String, uptime: String) -> Self {
        StatusResult { ping, uptime }
    }
}

pub fn get_status(uptime_command: &UptimeCommand) -> String {
    let uptime_result = uptime_command.uptime();
    let status_result = StatusResult::new("some_ping".to_string(), uptime_result);
    // Serialize to JSON
    serde_json::to_string(&status_result).unwrap()
}
