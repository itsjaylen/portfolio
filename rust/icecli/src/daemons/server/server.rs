use std::{fs::File, process, thread, time::Duration};

use daemonize::Daemonize;

use crate::{
    commands::server::args::ServerError, daemons::server::core::ipc::start_ipc_server,
};

use super::utils::{control::read_pid_file, process::process_running};

pub fn start_daemon() -> Result<(), ServerError> {
    // Check if PID file exists and the process is running
    if let Some(pid) = read_pid_file("/tmp/icecli.pid") {
        if process_running(pid) {
            println!("Daemon is already running.");
            // Return an error if the daemon is already running
            return Err(ServerError::DaemonAlreadyRunning);
        }
    }

    // Create stdout and stderr log files
    let stdout = File::create("/tmp/icecli.log").map_err(|_| ServerError::LogFileCreationFailed)?;
    let stderr = stdout
        .try_clone()
        .map_err(|_| ServerError::LogFileCreationFailed)?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/icecli.pid")
        .stdout(stdout)
        .stderr(stderr)
        .chown_pid_file(true)
        .privileged_action(|| println!("Privileged action"));

    // Attempt to start the daemon
    match daemonize.start() {
        Ok(_) => {
            println!("Daemon started successfully. PID: {}", process::id());

            thread::spawn(|| {
                match start_ipc_server() {
                    Ok(_) => {
                        // Successfully started the IPC server
                        println!("IPC server started successfully");
                    }
                    Err(e) => {
                        // Handle the error (e.g., log it, panic, or take other appropriate action)
                        eprintln!("Failed to start IPC server: {}", e);
                    }
                }
            });

            // Infinite loop for the daemon to run
            loop {
                println!("CLI Daemon is running...");
                thread::sleep(Duration::from_secs(5));
            }
        }
        Err(e) => {
            eprintln!("Error starting daemon: {}", e);
            // Return the error as ServerError
            return Err(ServerError::DaemonStartError(e.to_string()));
        }
    }
}
