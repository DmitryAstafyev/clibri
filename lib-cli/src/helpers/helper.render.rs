pub const SPACES_TAB: usize = 4;

pub fn tabs(num: usize) -> String {
    " ".repeat(num * SPACES_TAB)
}

pub fn inject_tabs(num: usize, input: String) -> String {
    let mut output: String = String::new();
    let lines = input.split('\n').collect::<Vec<&str>>();
    for (pos, line) in lines.iter().enumerate() {
        output = format!("{}{}{}{}",
            output,
            tabs(num),
            line,
            if pos < lines.len() - 1 { "\n" } else { "" }
        );
    }
    output
}