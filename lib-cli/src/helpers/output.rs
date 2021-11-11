const MAX_KEY_LEN: usize = 40;

pub fn keys(out: &str) -> String {
    let left = if MAX_KEY_LEN < out.len() { 0 } else { MAX_KEY_LEN - out.len() };
    format!("    {}{}", out, " ".repeat(left))
}

pub fn desk(out: &str) -> String {
    format!(" - {}", out)
}