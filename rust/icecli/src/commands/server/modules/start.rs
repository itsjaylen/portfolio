use crate::daemons::server::server::start_daemon;



// Make send signal instead
pub fn start_server() {
    let _ = start_daemon();
}
