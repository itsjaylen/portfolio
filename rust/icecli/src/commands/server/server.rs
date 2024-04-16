use crate::daemons::server::{server::start_daemon, utils::control::stop_daemon};

use super::{
    args::{ServerCommand, ServerError, ServerSubcommand},
    modules::logs::check_server_logs,
};

// Handles all sever subcommands
pub fn handle_server_command(server: ServerCommand) -> Result<(), ServerError> {
    let command = server.command;
    match command {
        ServerSubcommand::StartServer => todo!(),
        ServerSubcommand::StopServer => todo!(),
        ServerSubcommand::ShowServerLogs(logs) => {
            todo!()
        }
        ServerSubcommand::StartDaemon => {
            if let Err(err) = start_daemon() {
                let error_message = err.to_string();
                return Err(ServerError::DaemonStartError(error_message));
            }

            Ok(())
        }
        ServerSubcommand::StopDaemon => Ok(stop_daemon()),
        ServerSubcommand::ShowDaemonLogs(logs) => {
            let log_amount = logs.log_amount;
            if let Err(err) = check_server_logs(log_amount) {
                let error_message = err.to_string();
                return Err(ServerError::DaemonLogsError(error_message));
            }

            Ok(())
        }
    }
}
