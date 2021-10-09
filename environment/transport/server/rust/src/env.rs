use std::env::var_os;

mod constants {
    pub const DEBUG_MODE: &str = "FIBER_WS_SERVER_DEBUG_MODE";
}
pub fn is_debug_mode() -> bool {
    if let Some(mode) = var_os(constants::DEBUG_MODE) {
        let mode_str = mode.to_string_lossy().to_string();
        mode_str.to_ascii_lowercase() == "true"
            || mode_str.to_ascii_lowercase() == "on"
            || mode_str.to_ascii_lowercase() == "1"
    } else {
        false
    }
}
