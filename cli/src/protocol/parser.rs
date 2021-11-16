pub mod entities;
pub mod enums;
pub mod fields;
pub mod groups;
pub mod store;
pub mod structs;
pub mod types;

use super::{helpers, stop};
use entities::Entities;
use enums::Enum;
use fields::Field;
use groups::Group;
use helpers::{chars, hash};
use std::fs;
use std::path::PathBuf;
use store::Store;
use structs::Struct;
use types::PrimitiveTypes;

#[derive(Debug, Clone)]
enum ENext {
    Word((String, usize, Option<char>)),
    OpenStruct(usize),
    CloseStruct(usize),
    Semicolon(usize),
    Repeated(usize),
    Optional(usize),
    PathSpliter(usize),
    End(),
}

enum ENextErr {
    NotAscii(String),
    NumericFirst(),
    NotSupported(String),
}

#[derive(Debug, PartialEq)]
enum EExpectation {
    GroupDef,
    GroupName,
    StructDef,
    EnumDef,
    EnumValue,
    FieldType,
    FieldName,
    FieldRepeatedMark,
    FieldOptionalMark,
    StructName,
    EnumName,
    EntityOpen,
    EntityClose,
    Semicolon,
    PathSpliter,
}

pub struct Parser {
    src: PathBuf,
    prev: Option<ENext>,
}

impl Parser {
    pub fn new(src: PathBuf) -> Parser {
        Parser { src, prev: None }
    }

    pub fn parse(&mut self) -> Result<Store, Vec<String>> {
        fn is_in(src: &[EExpectation], target: &EExpectation) -> bool {
            src.iter().any(|e| e == target)
        }
        let mut content: String = match self.get_content(self.src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let mut errs: Vec<String> = vec![];
        let mut expectation: Vec<EExpectation> = vec![
            EExpectation::StructDef,
            EExpectation::GroupDef,
            EExpectation::EnumDef,
        ];
        let mut store: Store = Store::new(hash::get(&self.src).map_err(|e| vec![e])?);
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    self.prev = Some(enext.clone());
                    let offset: usize = match enext {
                        ENext::Word((word, offset, next_char)) => {
                            let next_char: char = if let Some(c) = next_char { c } else { '.' };
                            if Entities::get_entity(&word).is_some()
                                && (is_in(&expectation, &EExpectation::GroupDef)
                                    || is_in(&expectation, &EExpectation::StructDef)
                                    || is_in(&expectation, &EExpectation::EnumDef))
                            {
                                match Entities::get_entity(&word) {
                                    Some(Entities::EEntities::EGroup) => {
                                        if is_in(&expectation, &EExpectation::GroupDef) {
                                            expectation = vec![EExpectation::GroupName];
                                        } else {
                                            stop!(
                                                "Has been gotten Group Def, but expections is {:?}",
                                                expectation
                                            );
                                        }
                                    }
                                    Some(Entities::EEntities::EStruct) => {
                                        if is_in(&expectation, &EExpectation::StructDef) {
                                            expectation = vec![EExpectation::StructName];
                                        } else {
                                            stop!("Has been gotten Struct Def, but expections is {:?}", expectation);
                                        }
                                    }
                                    Some(Entities::EEntities::EEnum) => {
                                        if is_in(&expectation, &EExpectation::EnumDef) {
                                            expectation = vec![EExpectation::EnumName];
                                        } else {
                                            stop!(
                                                "Has been gotten Enum Def, but expections is {:?}",
                                                expectation
                                            );
                                        }
                                    }
                                    None => {
                                        stop!(
                                            "Has been gotten unkonwn definition {:?}",
                                            Entities::get_entity(&word)
                                        );
                                    }
                                };
                                if is_in(&expectation, &EExpectation::StructDef) {
                                    expectation = vec![EExpectation::StructName];
                                } else if is_in(&expectation, &EExpectation::EnumDef) {
                                    expectation = vec![EExpectation::EnumName];
                                } else if is_in(&expectation, &EExpectation::GroupDef) {
                                    expectation = vec![EExpectation::GroupName];
                                }
                            } else if is_in(&expectation, &EExpectation::StructName) {
                                store.open_struct(word.to_string());
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::EnumName) {
                                store.open_enum(word.to_string());
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::GroupName) {
                                store.open_group(word.to_string());
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::FieldName) {
                                if store.is_enum_opened() {
                                    store.set_enum_name(&word);
                                    expectation = vec![EExpectation::Semicolon];
                                } else {
                                    store.set_field_name(&word);
                                    expectation = vec![
                                        EExpectation::Semicolon,
                                        EExpectation::FieldOptionalMark,
                                    ];
                                }
                            } else if is_in(&expectation, &EExpectation::FieldType) {
                                if store.is_enum_opened() {
                                    if next_char == ';' {
                                        store.set_simple_enum_item(&word);
                                        expectation = vec![EExpectation::Semicolon];
                                    } else {
                                        store.set_enum_type(&word);
                                        expectation = vec![
                                            EExpectation::FieldName,
                                            EExpectation::FieldRepeatedMark,
                                            EExpectation::PathSpliter,
                                        ];
                                    }
                                } else {
                                    store.set_field_type(&word);
                                    expectation = vec![
                                        EExpectation::FieldName,
                                        EExpectation::FieldRepeatedMark,
                                        EExpectation::PathSpliter,
                                    ];
                                }
                            } else {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value {}",
                                    expectation, word
                                ));
                                break;
                            }
                            offset
                        }
                        ENext::OpenStruct(offset) => {
                            if !is_in(&expectation, &EExpectation::EntityOpen) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: OpenStruct",
                                    expectation
                                ));
                                break;
                            }
                            expectation = vec![
                                EExpectation::FieldType,
                                EExpectation::GroupDef,
                                EExpectation::StructDef,
                                EExpectation::EnumDef,
                                EExpectation::EnumValue,
                                EExpectation::EntityClose,
                            ];
                            store.open();
                            offset
                        }
                        ENext::CloseStruct(offset) => {
                            if !is_in(&expectation, &EExpectation::EntityClose) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: CloseStruct",
                                    expectation
                                ));
                                break;
                            }
                            expectation = vec![
                                EExpectation::FieldType, // Only if it's nested struct
                                EExpectation::GroupDef,
                                EExpectation::StructDef,
                                EExpectation::EnumDef,
                                EExpectation::EntityClose,
                            ];
                            store.close();
                            offset
                        }
                        ENext::Semicolon(offset) => {
                            if !is_in(&expectation, &EExpectation::Semicolon) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: Semicolon",
                                    expectation
                                ));
                                break;
                            }
                            if !store.is_enum_opened() {
                                store.close_field();
                            }
                            expectation = vec![
                                EExpectation::FieldType,
                                EExpectation::StructDef,
                                EExpectation::EnumDef,
                                EExpectation::EnumValue,
                                EExpectation::EntityClose,
                            ];
                            offset
                        }
                        ENext::Repeated(offset) => {
                            if !is_in(&expectation, &EExpectation::FieldRepeatedMark) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: FieldRepeatedMark",
                                    expectation
                                ));
                                break;
                            }
                            expectation = vec![EExpectation::FieldName];
                            store.set_field_type_as_repeated();
                            offset
                        }
                        ENext::Optional(offset) => {
                            if !is_in(&expectation, &EExpectation::FieldOptionalMark) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: FieldOptionalMark",
                                    expectation
                                ));
                                break;
                            }
                            expectation = vec![EExpectation::Semicolon];
                            store.set_field_type_as_optional();
                            offset
                        }
                        ENext::PathSpliter(offset) => {
                            if !is_in(&expectation, &EExpectation::PathSpliter) {
                                errs.push(format!(
                                    "Unexpecting next step: {:?}. Value: PathSpliter",
                                    expectation
                                ));
                                break;
                            }
                            if !store.is_field_opened() && !store.is_enum_opened() {
                                errs.push(
                                    "Unexpecting : as soon as no open field or enum".to_owned(),
                                );
                            }
                            expectation = vec![EExpectation::FieldType];
                            offset
                        }
                        ENext::End() => {
                            break;
                        }
                    };
                    content = String::from(&content[offset..]);
                }
                Err(e) => {
                    match e {
                        ENextErr::NotAscii(msg) => errs.push(format!("ASCII error: {}", msg)),
                        ENextErr::NotSupported(msg) => {
                            errs.push(format!("Not supported char(s) error: {}", msg))
                        }
                        ENextErr::NumericFirst() => errs
                            .push("Numeric symbols cannot be used as first in names.".to_string()),
                    };
                    return Err(errs);
                }
            }
        }
        if errs.is_empty() {
            match store.order() {
                Ok(_) => Ok(store),
                Err(e) => Err(vec![e]),
            }
        } else {
            Err(errs)
        }
    }

    fn next(&mut self, content: String) -> Result<ENext, ENextErr> {
        let mut str: String = String::new();
        let mut pass: usize = 0;
        let break_chars: Vec<char> = vec![
            chars::SEMICOLON,
            chars::OPEN,
            chars::CLOSE,
            chars::QUESTION,
            chars::DOT,
        ];
        let special_chars: Vec<char> = vec![chars::OPEN_SQ_BRACKET, chars::CLOSE_SQ_BRACKET];
        let allowed_chars: Vec<char> = vec![chars::UNDERLINE];
        for char in content.chars() {
            pass += 1;
            if !char.is_ascii() {
                return Err(ENextErr::NotAscii(format!(
                    "found not ascii char: {}",
                    char
                )));
            }
            if char.is_ascii_digit() && str.is_empty() {
                return Err(ENextErr::NumericFirst());
            }
            if char.is_ascii_whitespace() && str.is_empty() {
                continue;
            }
            let mut breakable: Option<char> = None;
            if break_chars.iter().any(|&c| c == char) {
                breakable = Some(char);
            }
            if breakable.is_some() && str.is_empty() {
                match char {
                    chars::SEMICOLON => return Ok(ENext::Semicolon(pass)),
                    chars::OPEN => return Ok(ENext::OpenStruct(pass)),
                    chars::CLOSE => return Ok(ENext::CloseStruct(pass)),
                    chars::QUESTION => return Ok(ENext::Optional(pass)),
                    chars::DOT => return Ok(ENext::PathSpliter(pass)),
                    _ => {}
                };
            }
            let special: bool = special_chars.iter().any(|&c| c == char);
            if special {
                match char {
                    chars::OPEN_SQ_BRACKET => {
                        if !str.is_empty() {
                            breakable = Some(char);
                        } else {
                            continue;
                        }
                    }
                    chars::CLOSE_SQ_BRACKET => {
                        if let Some(c) = str.chars().next() {
                            if c != '[' {
                                return Err(ENextErr::NotSupported(format!(
                                    "found not supportable char: {}",
                                    char
                                )));
                            }
                        }
                        return Ok(ENext::Repeated(pass));
                    }
                    _ => {}
                };
            }
            if char.is_ascii_whitespace() || breakable.is_some() {
                return Ok(ENext::Word((str, pass - 1, breakable)));
            }
            let allowed: bool = allowed_chars.iter().any(|&c| c == char);
            if !char.is_ascii_alphanumeric() && !allowed {
                return Err(ENextErr::NotSupported(format!(
                    "found not supportable char: {}",
                    char
                )));
            }
            str.push(char);
        }
        if str.is_empty() {
            Ok(ENext::End())
        } else {
            Ok(ENext::Word((str, pass - 1, None)))
        }
    }

    pub fn get_content(&self, target: PathBuf) -> Result<String, String> {
        if !target.exists() {
            Err(format!(
                "File {} doesn't exists",
                target.as_path().display().to_string()
            ))
        } else {
            match fs::read_to_string(target.as_path()) {
                Ok(content) => Ok(content),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
