

use std::time::Instant;

use crate::daemons::server::core::ipc::Command;


impl Command {
    pub fn ping(&self) -> String {
        let start_time = Instant::now();
        format!("{} ms\n", start_time.elapsed().as_millis())
    }
}
