use nix::unistd::Pid;

pub fn process_running(pid: i32) -> bool {
  use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};

  match waitpid(Pid::from_raw(pid), Some(WaitPidFlag::WNOHANG)) {
      Ok(WaitStatus::Exited(_, _)) => false, // Process has exited
      Ok(_) => true,                         // Process is still running
      Err(_) => false,                       // Error occurred, assuming process is not running
  }
}