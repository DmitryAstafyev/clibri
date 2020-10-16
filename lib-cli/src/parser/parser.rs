use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };
use std::fs;
use std::str::{ Chars };
use types::{ PrimitiveTypes };
use entities::{ Entities };

#[path = "./parser.types.rs"]
pub mod types;

#[path = "./parser.entities.rs"]
pub mod entities;

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

#[derive(Debug)]
enum EExpectation {
    Word,
    EntityDef,
    StructDef,
    EnumDef,
    EnumValue,
    Type,
    FieldName,
    EntityName,
    EntityOpen,
    EntityClose,
    Semicolon,
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
        let mut errs: Vec<String> = vec![];
        let mut expectation: EExpectation = EExpectation::EntityDef;
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    let offset: usize = match enext {
                        ENext::Word((word, offset)) => {
                            match expectation {
                                EExpectation::EntityDef => {
                                    if let Some(entity) = Entities::get_entity(&word) {
                                        println!("Found entity: {:?}", entity);
                                        expectation = EExpectation::EntityName;
                                    } else {
                                        errs.push(format!("Expecting {:?}. Value {}", EExpectation::EntityDef, word));
                                        break;
                                    }
                                },
                                EExpectation::EntityName => {
                                    expectation = EExpectation::EntityOpen;
                                },
                                EExpectation::FieldName => {
                                    expectation = EExpectation::Type;
                                },
                                _ => {
                                    errs.push(format!("Unexpecting next step: {:?}. Value {}", expectation, word));
                                    break;
                                }
                            }
                            println!("Word: {}", word);
                            offset
                        },
                        ENext::OpenStruct(offset) => {
                            if let EExpectation::EntityOpen = expectation {
                                expectation = EExpectation::Word;
                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value: OpenStruct", expectation));
                                break;
                            }
                            println!("open");
                            offset
                        },
                        ENext::CloseStruct(offset) => {
                            if let EExpectation::EntityClose = expectation {

                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value: CloseStruct", expectation));
                                break;
                            }
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
                    // errs.push(e);
                    return Err(errs);
                },
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
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