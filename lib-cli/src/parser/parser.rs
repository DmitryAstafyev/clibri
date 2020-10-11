use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };
use std::fs;
use std::str::{Chars};

enum ENext {
    Word((String, usize)),
    OpenStruct(usize),
    CloseStruct(usize),
    Semicolon(usize),
    Space(usize),
    End(),
}

enum ENextErr {
    NotAscii(String),
    NumericFirst(),
    NotSupported(String),
}

pub struct Parser {
    _src: PathBuf,
    _rs: PathBuf,
    _ts: PathBuf,
}

impl Parser {

    fn new(src: PathBuf, rs: PathBuf, ts: PathBuf) -> Parser {
        Parser { _src: src, _rs: rs, _ts: ts }
    }

    fn parse(&mut self) -> Result<(), Vec<String>> {
        let mut content: String = match self.get_content(self._src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let errs: Vec<String> = vec![];
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    let offset: usize = match enext {
                        ENext::Word((word, offset)) => {
                            println!("Word: {}", word);
                            offset
                        },
                        ENext::OpenStruct(offset) => {
                            println!("open");
                            offset
                        },
                        ENext::CloseStruct(offset) => {
                            println!("close");
                            offset
                        },
                        ENext::Semicolon(offset) => {
                            println!(";");
                            offset
                        },
                        ENext::Space(offset) => {
                            println!("space");
                            offset
                        },
                        ENext::End() => {
                            println!("end");
                            break;
                        },
                    };
                    content = String::from(&content[offset..]);
                },
                Err(e) => {
                    println!("POINT 2");
                    return Err(errs);
                },
            }
        }
        Err(vec![])
        // Ok(())
    }

    fn next(&mut self, content: String) -> Result<ENext, ENextErr> {
        let mut str: String = String::new();
        let mut pass: usize = 0;
        let break_chars: Vec<char> = vec![';', '{', '}'];
        let allowed_chars: Vec<char> = vec!['_'];
        for char in content.chars() {
            pass += 1;
            if !char.is_ascii() {
                return Err(ENextErr::NotAscii(format!("found not ascii char: {}", char)))
            }
            if char.is_ascii_digit() && str.is_empty() {
                return Err(ENextErr::NumericFirst());
            }
            if char.is_ascii_whitespace() && str.is_empty() {
                continue;
            }
            let breakable: bool = break_chars.iter().any(|&c| c == char);
            if breakable && str.is_empty() {
                match char {
                    ';' => return Ok(ENext::Semicolon(pass)),
                    '{' => return Ok(ENext::OpenStruct(pass)),
                    '}' => return Ok(ENext::CloseStruct(pass)),
                    _ => {}
                };
            }
            if char.is_ascii_whitespace() || breakable {
                return Ok(ENext::Word((str, pass - 1)));
            }
            let allowed: bool = allowed_chars.iter().any(|&c| c == char);
            if !char.is_ascii_alphanumeric() && !allowed {
                return Err(ENextErr::NotSupported(format!("found not supportable char: {}", char)))
            }
            str.push(char);
        }
        if str.is_empty() {
            Ok(ENext::End())
        } else {
            Ok(ENext::Word((str, pass - 1)))
        }
    }

    fn get_content(&self, target: PathBuf) -> Result<String, String> {
        if !target.exists() {
            Err(format!("File {} doesn't exists", target.as_path().display().to_string()))
        } else {
            match fs::read_to_string(target.as_path()) {
                Ok(content) => Ok(content),
                Err(e) => Err(e.to_string())
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use std::path::{ PathBuf, Path };
    use super::{Parser};

    #[test]
    fn parsing() {
        let src = Path::new("/Users/dmitry.astafyev/projects/fiber/lib-cli/test/protocol.prot");
        let ts = Path::new("/Users/dmitry.astafyev/projects/fiber/lib-cli/test/protocol.prot.ts");
        let rs = Path::new("/Users/dmitry.astafyev/projects/fiber/lib-cli/test/protocol.prot.rs");
        let mut parser: Parser = Parser::new(src.to_path_buf(), rs.to_path_buf(), ts.to_path_buf());
        match parser.parse() {
            Ok(buf) => {
                assert_eq!(true, true);
            },
            Err(e) => {
                // println!("{}", e);
                assert_eq!(true, false);
            }
        }
    }

}