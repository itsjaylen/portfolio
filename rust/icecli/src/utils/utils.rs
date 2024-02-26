use std::env;

pub fn get_home_directory() -> String {
    env::var("HOME").unwrap_or_else(|_| ".".to_string())
}
pub fn is_debug_mode() -> bool {
    env::var("DEBUG").unwrap_or_default().parse().unwrap_or(false)
}
