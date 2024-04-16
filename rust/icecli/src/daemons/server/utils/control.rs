use std::{fs, process, thread, time::Duration};

use nix::{sys::signal::{kill, Signal}, unistd::Pid};

use crate::daemons::server::utils::process::process_running;

pub fn stop_daemon() {
  if let Some(pid) = read_pid_file("/tmp/icecli.pid") {
      if let Err(err) = kill(Pid::from_raw(pid), Signal::SIGTERM) {
          eprintln!("Error stopping daemon: {}", err);
          process::exit(1);
      }
      // Wait for the daemon to exit properly
      thread::sleep(Duration::from_secs(1));
      // Check if the process is still running after SIGTERM
      if process_running(pid) {
          eprintln!("Failed to stop daemon. Try again or use force kill.");
          process::exit(1);
      }
      println!("Daemon stopped successfully.");
      // Remove the PID file
      if let Err(err) = fs::remove_file("/tmp/icecli.pid") {
          eprintln!("Error removing PID file: {}", err);
      }
  } else {
      println!("Daemon is not running.");
  }
}

pub fn read_pid_file(path: &str) -> Option<i32> {
  match fs::read_to_string(path) {
      Ok(content) => {
          if let Ok(pid) = content.trim().parse::<i32>() {
              Some(pid)
          } else {
              None
          }
      }
      Err(_) => None,
  }
}