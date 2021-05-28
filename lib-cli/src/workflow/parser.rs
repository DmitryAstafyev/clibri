
#[path = "./parser.config.rs"]
pub mod config;

#[path = "./parser.request.rs"]
pub mod request;

#[path = "./parser.event.rs"]
pub mod event;

use super::render::{Target};
use std::fs;
use std::path::PathBuf;
use config::{Config};
use request::{Request};
use event::{Event};

pub mod chars {
    pub const DOT: char = '.';
    pub const SEMICOLON: char = ';';
    pub const OPEN: char = '{';
    pub const CLOSE: char = '}';
    pub const QUESTION: char = '?';
    pub const COLON: char = ':';
    pub const AMPERSAND: char = '&';
    pub const AT: char = '@';
    pub const UNDERLINE: char = '_';
    pub const NUMBER: char = '#';
    pub const OPEN_BRACKET: char = '(';
    pub const CLOSE_BRACKET: char = ')';
    pub const ARROW: char = '>';
    pub const EXCLAMATION: char = '!';
    pub const CARET: char = '\n';
}

pub trait EntityParser {
    
    fn open(word: String) -> Option<Self> where Self: Sized;
    fn next(&mut self, entity: ENext) -> Result<usize, String>;
    fn closed(&self) -> bool;
    fn print(&self);

}

#[derive(Debug, Clone)]
pub enum ENext {
    Word((String, usize, Option<char>)),
    Open(usize),
    Close(usize),
    OpenBracket(usize),
    CloseBracket(usize),
    Arrow(usize),
    Exclamation(usize),
    Semicolon(usize),
    PathDelimiter(usize),
    ValueDelimiter(usize),
    End(),
}

enum ENextErr {
    NotAscii(String),
    NumericFirst(),
    NotSupported(String),
    InvalidSyntax(String),
}

pub struct Parser {
    src: PathBuf,
    cursor: usize,
    content: String,
}

impl Parser {
    pub fn new(src: PathBuf) -> Parser {
        Self {
            src,
            cursor: 0,
            content: String::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let mut content: String = match self.get_content(self.src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        self.content = content.clone();
        let mut opened: Option<Box<dyn EntityParser>> = None;
        loop {
            match self.next(content.clone()) {
                Ok(enext) => {
                    let mut offset: usize = match enext.clone() {
                        ENext::Word((word, offset, _)) => if opened.is_none() {
                            if let Some(entity) = Config::open(word.clone()) {
                                opened = Some(Box::new(entity));
                            } else if let Some(entity) = Request::open(word.clone()) {
                                opened = Some(Box::new(entity));
                            } else if let Some(entity) = Event::open(word.clone()) {
                                opened = Some(Box::new(entity));
                            }
                            if opened.is_none() {
                                return Err(format!("Unknown keyword {}", word));
                            }
                            offset
                        } else {
                            0
                        },
                        ENext::End() => {
                            break;
                        },
                        _ => {
                            0
                        },
                    };
                    if offset == 0 {
                        offset = if let Some(entity) = opened.as_deref_mut() {
                            match entity.next(enext.clone()) {
                                Ok(offset) => {
                                    if entity.closed() {
                                        entity.print();
                                        // TODO: Save to store
                                        opened = None;
                                        println!("DROPPED");
                                    }
                                    offset
                                },
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        } else {
                            return Err(format!("Fail to find any open entities. State: {:?}", enext));
                        };
                    }
                    self.cursor += offset;
                    content = String::from(&content[offset..]);
                }
                Err(e) => {
                    return Err(match e {
                        ENextErr::NotAscii(msg) => format!("ASCII error: {}", msg),
                        ENextErr::NotSupported(msg) => format!("Not supported char(s) error: {}", msg),
                        ENextErr::InvalidSyntax(msg) => format!("Invalid syntax error: {}", msg),
                        ENextErr::NumericFirst() =>"Numeric symbols cannot be used as first in names.".to_string(),
                    });
                }
            };
            // break;
        }
        Err(String::from(""))
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

    fn err() {

    }

    fn next(&mut self, content: String) -> Result<ENext, ENextErr> {
        let mut str: String = String::new();
        let mut pass: usize = 0;
        let break_chars: Vec<char> = vec![
            chars::CLOSE,
            chars::OPEN,
            chars::COLON,
            chars::QUESTION,
            chars::SEMICOLON,
            chars::DOT,
            chars::OPEN_BRACKET,
            chars::CLOSE_BRACKET,
            chars::ARROW,
            chars::EXCLAMATION,
        ];
        let allowed_chars: Vec<char> = vec![
            chars::UNDERLINE,
        ];
        let limited_chars: Vec<char> = vec![
            chars::AT,
            chars::AMPERSAND,
        ];
        let mut limited: bool = false;
        let mut comment: bool = false;
        for char in content.chars() {
            pass += 1;
            if comment && char == chars::CARET {
                comment = false;
            } else if comment {
                continue;
            } else if char == chars::NUMBER {
                comment = true;
                continue;
            }
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
                    chars::OPEN => return Ok(ENext::Open(pass)),
                    chars::CLOSE => return Ok(ENext::Close(pass)),
                    chars::DOT => return Ok(ENext::PathDelimiter(pass)),
                    chars::COLON => return Ok(ENext::ValueDelimiter(pass)),
                    chars::OPEN_BRACKET => return Ok(ENext::OpenBracket(pass)),
                    chars::CLOSE_BRACKET => return Ok(ENext::CloseBracket(pass)),
                    chars::ARROW => return Ok(ENext::Arrow(pass)),
                    chars::EXCLAMATION => return Ok(ENext::Exclamation(pass)),
                    _ => {}
                };
            }
            if char.is_ascii_whitespace() || breakable.is_some() {
                if limited && 
                    ((!str.starts_with(chars::AMPERSAND) && str.contains(chars::AMPERSAND)) || 
                    (!str.starts_with(chars::AT) && str.contains(chars::AT)))
                {
                    return Err(ENextErr::InvalidSyntax(format!(
                        "Chars {} and {} can be used only at the begging of words. Issue: {}",
                        chars::AMPERSAND, chars::AT, str
                    )));
                } else {
                    return Ok(ENext::Word((str, pass - 1, breakable)));
                }
            }
            let allowed: bool = allowed_chars.iter().any(|&c| c == char);
            limited = if limited {
                true
            } else {
                limited_chars.iter().any(|&c| c == char)
            };
            if !char.is_ascii_alphanumeric() && !allowed && !limited {
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
}
