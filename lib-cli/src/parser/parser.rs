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

#[derive(Debug, PartialEq)]
enum EExpectation {
    StructDef,
    EnumDef,
    EnumValue,
    FieldType,
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
        fn is_in(src: &Vec<EExpectation>, target: &EExpectation) -> bool {
            src.iter().position(|e| e == target).is_some()
        }
        let mut content: String = match self.get_content(self._src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let mut errs: Vec<String> = vec![];
        let mut expectation: Vec<EExpectation> = vec![EExpectation::StructDef];
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    let offset: usize = match enext {
                        ENext::Word((word, offset)) => {
                            if Entities::get_entity(&word).is_some() && 
                               (is_in(&expectation, &EExpectation::StructDef) ||
                                is_in(&expectation, &EExpectation::EnumDef)) {                                    
                                println!("Found entity: {:?}", Entities::get_entity(&word));
                                expectation = vec![EExpectation::EntityName];
                            } else if is_in(&expectation, &EExpectation::EntityName) {
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::FieldName) {
                                expectation = vec![EExpectation::Semicolon];
                            } else if is_in(&expectation, &EExpectation::FieldType) {
                                if let Some(primitive) = PrimitiveTypes::get_entity(&word) {
                                    println!("Found type: {:?}", primitive);
                                    expectation = vec![EExpectation::FieldName];
                                } else {
                                    errs.push(format!("Expecting {:?}. Value {}", EExpectation::FieldType, word));
                                    break;
                                }
                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value {}", expectation, word));
                                break;
                            }
                            println!("Word: {}", word);
                            offset
                        },
                        ENext::OpenStruct(offset) => {
                            if is_in(&expectation, &EExpectation::EntityOpen) {
                                expectation = vec![
                                    EExpectation::FieldType,
                                    EExpectation::StructDef,
                                    EExpectation::EnumDef
                                ];
                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value: OpenStruct", expectation));
                                break;
                            }
                            println!("open");
                            offset
                        },
                        ENext::CloseStruct(offset) => {
                            if is_in(&expectation, &EExpectation::EntityClose) {

                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value: CloseStruct", expectation));
                                break;
                            }
                            println!("close");
                            offset
                        },
                        ENext::Semicolon(offset) => {
                            if !is_in(&expectation, &EExpectation::Semicolon) {
                                errs.push(format!("Unexpecting next step: {:?}. Value: Semicolon", expectation));
                                break;
                            }
                            expectation = vec![
                                EExpectation::FieldType,
                                EExpectation::StructDef,
                                EExpectation::EnumDef,
                                EExpectation::EntityClose
                            ];
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
                    match e {
                        ENextErr::NotAscii(msg) => errs.push(format!("ASCII error: {}", msg)),
                        ENextErr::NotSupported(msg) => errs.push(format!("Not supported char(s) error: {}", msg)),
                        ENextErr::NumericFirst() => errs.push("Numeric symbols cannot be used as first in names.".to_string()),
                    };
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
        if let Ok(exe) = std::env::current_exe() {
            if let Some(path) = exe.as_path().parent() {
                let src = path.join("../../../test/protocol.prot");
                let ts = path.join("../../../test/protocol.prot.ts");
                let rs = path.join("../../../test/protocol.prot.rs");
                let mut parser: Parser = Parser::new(src.to_path_buf(), rs.to_path_buf(), ts.to_path_buf());
                match parser.parse() {
                    Ok(buf) => {
                        assert_eq!(true, true);
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