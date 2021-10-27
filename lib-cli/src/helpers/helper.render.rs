pub const SPACES_TAB: usize = 4;

pub fn tabs(num: usize) -> String {
    " ".repeat(num * SPACES_TAB)
}

pub fn inject_tabs(num: usize, input: String) -> String {
    let mut output: String = String::new();
    let lines = input.split('\n').collect::<Vec<&str>>();
    for (pos, line) in lines.iter().enumerate() {
        output = format!(
            "{}{}{}{}",
            output,
            tabs(num),
            line,
            if pos < lines.len() - 1 { "\n" } else { "" }
        );
    }
    output
}

pub fn inject_tabs_except(num: usize, input: String, exceptions: Vec<usize>) -> String {
    let mut output: String = String::new();
    let lines = input.split('\n').collect::<Vec<&str>>();
    for (pos, line) in lines.iter().enumerate() {
        if exceptions.iter().any(|i| i == &pos) {
            output = format!(
                "{}{}{}",
                output,
                line,
                if pos < lines.len() - 1 { "\n" } else { "" }
            );
        } else {
            output = format!(
                "{}{}{}{}",
                output,
                tabs(num),
                line,
                if pos < lines.len() - 1 { "\n" } else { "" }
            );
        }
    }
    output
}

pub fn into_rust_path(input: &str) -> String {
    input.to_string().replace(".", "::")
}
pub fn into_ts_path(input: &str) -> String {
    input.to_string().to_lowercase()
}
