#[path = "./ctrl.args.rs"]
pub mod ctrlargs;

#[path = "./helpers/helpers.rs"]
pub mod helpers;

#[path = "./parser/parser.rs"]
pub mod parser;

#[path = "./writer/writer.rs"]
pub mod writer;

fn main() {
    let ctrl: ctrlargs::CtrlArgs = ctrlargs::CtrlArgs::new();
    match ctrl.errors() {
        Ok(_) => {},
        Err(_) => std::process::exit(1),
    }
    if let Err(errors) = ctrl.actions() {
        println!("{}", errors.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::parser::{ Parser, EDest };
    use super::writer::{ rust };

    #[test]
    fn parsing() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(path) = exe.as_path().parent() {
                let src = path.join("../../../test/protocol.prot");
                let mut parser: Parser = Parser::new(src, vec![
                    EDest::Rust(path.join("../../../test/protocol.prot.rs")),
                    EDest::TypeScript(path.join("../../../test/protocol.prot.ts"))
                ]);
                match parser.parse() {
                    Ok(store) => {
                        println!("{}", rust::get_str(store));
                        assert_eq!(true, false);
                    },
                    Err(e) => {
                        println!("{}", e[0]);
                        assert_eq!(true, false);
                    }
                }        
            }
        }
    }

}
