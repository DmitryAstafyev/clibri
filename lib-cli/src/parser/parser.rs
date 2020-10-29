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
    StructName,
    EnumName,
    EntityOpen,
    EntityClose,
    Semicolon,
}

#[derive(Debug)]
struct PrimitiveField {
    id: usize,
    parent: usize,
    name: String,
    kind: String,
}

impl PrimitiveField {

    pub fn new(id: usize, parent: usize, kind: String) -> Self {
        PrimitiveField {
            id,
            parent,
            name: String::new(),
            kind,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_type(&mut self, kind: PrimitiveTypes::ETypes) {
        if let Some(primitive) = PrimitiveTypes::get_entity_as_string(kind) {
            self.kind = primitive;
        } else {
            panic!("Unknown type");
        }
    }

}

#[derive(Debug)]
enum EnumValue {
    StringValue(String),
    NumericValue(usize),
}

#[derive(Debug)]
struct EnumItem {
    name: String,
    value: EnumValue,
}

#[derive(Debug)]
struct Enum {
    id: usize,
    parent: usize,
    name: String,
    variants: Vec<EnumItem>,
}

impl Enum {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Enum {
            id,
            parent,
            name,
            variants: vec![],
        }
    }

}

#[derive(Debug)]
struct Struct {
    id: usize,
    parent: usize,
    name: String,
    fields: Vec<PrimitiveField>,
    structs: Vec<Struct>,
    enums: Vec<Enum>,
}

impl Struct {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Struct {
            id,
            parent,
            name,
            fields: vec![],
            structs: vec![],
            enums: vec![],
        }
    }

    pub fn find(&mut self, id: usize) -> Option<&mut Struct> {
        for child in self.structs.iter_mut() {
            if child.id == id {
                return Some(child);
            }
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn add_field(&mut self, mut field: PrimitiveField) {
        if self.fields.iter().any(|f| f.name == field.name) {
            panic!("Fail to add field \"{}\" into \"{}\" because field with same name already exist", field.name, self.name);
        }
        field.parent = self.id;
        self.fields.push(field);
    }

}

enum EStructValue {
    PrimitiveField(PrimitiveField),
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Store {
    sequence: usize,
    structs: Vec<Struct>,
    enums: Vec<Enum>,
    c_struct: Option<Struct>,
    c_enum: Option<Enum>,
    c_field: Option<PrimitiveField>,
    path: Vec<usize>,
}

impl Store {

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Store {
            sequence: 0,
            structs: vec![],
            enums: vec![],
            c_struct: None,
            c_enum: None,
            c_field: None,
            path: vec![],
        }
    }

    pub fn open_struct(&mut self, name: String) {
        let mut parent: usize = 0;
        if let Some(c_struct) = self.c_struct.take() {
            parent = c_struct.id;
        }
        self.sequence += 1;
        self.c_struct = Some(Struct::new(self.sequence, parent, name));
        self.path.push(self.sequence);
    }

    pub fn open_enum(&mut self, name: String) {
        let mut parent: usize = 0;
        if let Some(c_struct) = self.c_struct.take() {
            parent = c_struct.id;
            self.c_struct = Some(c_struct);
        }
        if self.c_enum.is_some() {
            panic!("Nested enums arn't supported");
        }
        self.sequence += 1;
        self.c_enum = Some(Enum::new(self.sequence, parent, name));
    }

    pub fn set_field_type(&mut self, type_str: &str) {
        if self.c_field.is_some() {
            panic!("Fail to create new field, while previous isn't closed.");
        }
        let mut parent: usize = 0;
        if let Some(c_struct) = self.c_struct.take() {
            parent = c_struct.id;
            self.c_struct = Some(c_struct);
        } else {
            panic!("Fail to create new field, because no open struct.");
        }
        if PrimitiveTypes::get_entity(type_str).is_some() {
            self.sequence += 1;
            self.c_field = Some(PrimitiveField::new(self.sequence, parent, type_str.to_string()));
        } else {
            panic!("Expecting {:?}. Value {}", EExpectation::FieldType, type_str)
        }
    }

    pub fn set_field_name(&mut self, name_str: &str) {
        if let Some(mut c_struct) = self.c_struct.take() {
            if let Some(mut c_field) = self.c_field.take() {
                c_field.set_name(name_str.to_string());
                c_struct.add_field(c_field);
                self.c_struct = Some(c_struct);
                self.c_field = None;
            } else {
                panic!("Fail to close field, while it wasn't opened.");
            }
        } else {
            panic!("Fail to close new field, because no open struct.");
        }
    }

    pub fn open(&mut self) {
        if self.c_struct.is_none() && self.c_enum.is_none() {
            panic!("No created struct or enum");
        }
        println!("open");
    }

    pub fn close(&mut self) {
        if self.c_struct.is_none() && self.c_enum.is_none() {
            panic!("No opened struct or enum");
        }
        println!("close");
    }

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
        fn is_in(src: &[EExpectation], target: &EExpectation) -> bool {
            src.iter().any(|e| e == target)
        }
        let mut content: String = match self.get_content(self._src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let mut errs: Vec<String> = vec![];
        let mut expectation: Vec<EExpectation> = vec![EExpectation::StructDef];
        let mut store: Store = Store::new();
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    let offset: usize = match enext {
                        ENext::Word((word, offset)) => {
                            if Entities::get_entity(&word).is_some() && 
                               (is_in(&expectation, &EExpectation::StructDef) || is_in(&expectation, &EExpectation::EnumDef)) {                                    
                                println!("Found entity: {:?}", Entities::get_entity(&word));
                                if is_in(&expectation, &EExpectation::StructDef) {
                                    expectation = vec![EExpectation::StructName];
                                } else if is_in(&expectation, &EExpectation::EnumDef) {
                                    expectation = vec![EExpectation::EnumName];
                                }
                            } else if is_in(&expectation, &EExpectation::StructName) {
                                store.open_struct(word.to_string());
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::EnumName) {
                                store.open_enum(word.to_string());
                                expectation = vec![EExpectation::EntityOpen];
                            } else if is_in(&expectation, &EExpectation::FieldName) {
                                store.set_field_name(&word);
                                expectation = vec![EExpectation::Semicolon];
                            } else if is_in(&expectation, &EExpectation::FieldType) {
                                store.set_field_type(&word);
                                expectation = vec![EExpectation::FieldName];
                            } else {
                                errs.push(format!("Unexpecting next step: {:?}. Value {}", expectation, word));
                                break;
                            }
                            println!("Word: {}", word);
                            offset
                        },
                        ENext::OpenStruct(offset) => {
                            if !is_in(&expectation, &EExpectation::EntityOpen) {
                                errs.push(format!("Unexpecting next step: {:?}. Value: OpenStruct", expectation));
                                break;
                            }
                            expectation = vec![
                                EExpectation::FieldType,
                                EExpectation::StructDef,
                                EExpectation::EnumDef
                            ];
                            store.open();
                            offset
                        },
                        ENext::CloseStruct(offset) => {
                            if !is_in(&expectation, &EExpectation::EntityClose) {
                                errs.push(format!("Unexpecting next step: {:?}. Value: CloseStruct", expectation));
                                break;
                            }
                            expectation = vec![
                                EExpectation::FieldType, // Only if it's nested struct
                                EExpectation::StructDef,
                                EExpectation::EnumDef
                            ];
                            store.close();
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
            println!("{:?}", store);
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