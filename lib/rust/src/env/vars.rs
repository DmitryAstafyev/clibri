pub fn debug_mode() -> Option<bool> {
    match std::env::var("FIBER_DEBUG_MODE") {
        Ok(value) => if value.to_ascii_lowercase() == "true" || value.to_ascii_lowercase() == "on"  || value.to_ascii_lowercase() == "1" {
            Some(true)
        } else {
            Some(false)
        }
        Err(_) => None
    }
}