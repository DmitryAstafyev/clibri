#[path = "./parser_config.rs"]
pub mod config;

#[path = "./parser_request.rs"]
pub mod request;

#[path = "./parser_event.rs"]
pub mod event;

#[path = "./parser_beacon.rs"]
pub mod beacon;

#[path = "./parser_store.rs"]
pub mod store;

use super::{
    helpers::{chars, hash},
    protocol::{
        fields::Field,
        store::{Store as Protocol, INTERNAL_SERVICE_GROUP},
        types::PrimitiveTypes,
    },
    render::Target,
};
use beacon::{Beacons, Broadcast};
use config::Config;
use event::Event;
use request::Request;
use std::fs;
use std::path::PathBuf;
use store::Store;

pub enum EntityOut {
    Config(Config),
    Event(Event),
    Request(Request),
    Beacons(Beacons),
}

pub trait EntityParser {
    fn open(word: String) -> Option<Self>
    where
        Self: Sized;
    fn next(&mut self, entity: ENext, protocol: &mut Protocol) -> Result<usize, String>;
    fn closed(&self) -> bool;
    fn print(&self);
    fn extract(&mut self) -> EntityOut;
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
    Question(usize),
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
    store: Store,
}

impl Parser {
    pub fn new(src: PathBuf) -> Result<Parser, String> {
        Ok(Self {
            src: src.clone(),
            cursor: 0,
            content: String::new(),
            store: Store::new(hash::get(&src)?),
        })
    }

    pub fn parse(&mut self, protocol: &mut Protocol) -> Result<Store, String> {
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
                        ENext::Word((word, offset, _)) => {
                            if opened.is_none() {
                                if let Some(entity) = Config::open(word.clone()) {
                                    opened = Some(Box::new(entity));
                                } else if let Some(entity) = Request::open(word.clone()) {
                                    opened = Some(Box::new(entity));
                                } else if let Some(entity) = Beacons::open(word.clone()) {
                                    opened = Some(Box::new(entity));
                                } else if let Some(entity) = Event::open(word.clone()) {
                                    opened = Some(Box::new(entity));
                                }
                                if opened.is_none() {
                                    return Err(self.err(&format!("Unknown keyword {}", word)));
                                }
                                offset
                            } else {
                                0
                            }
                        }
                        ENext::End() => {
                            break;
                        }
                        _ => 0,
                    };
                    if offset == 0 {
                        offset = if let Some(entity) = opened.as_deref_mut() {
                            match entity.next(enext.clone(), protocol) {
                                Ok(offset) => {
                                    if entity.closed() {
                                        if let Err(e) = match entity.extract() {
                                            EntityOut::Config(config) => {
                                                self.store.set_config(config)
                                            }
                                            EntityOut::Event(event) => self.store.add_event(event),
                                            EntityOut::Beacons(beacons) => {
                                                self.store.add_beacon(beacons)
                                            }
                                            EntityOut::Request(request) => {
                                                self.store.add_request(request)
                                            }
                                        } {
                                            return Err(self.err(&e));
                                        } else {
                                            opened = None;
                                        }
                                    }
                                    offset
                                }
                                Err(err) => {
                                    return Err(self.err(&err));
                                }
                            }
                        } else {
                            return Err(self.err(&format!(
                                "Fail to find any open entities. State: {:?}",
                                enext
                            )));
                        };
                    }
                    content = String::from(&content[offset..]);
                    self.cursor += offset;
                }
                Err(e) => {
                    return Err(match e {
                        ENextErr::NotAscii(msg) => self.err(&format!("ASCII error: {}", msg)),
                        ENextErr::NotSupported(msg) => {
                            self.err(&format!("Not supported char(s) error: {}", msg))
                        }
                        ENextErr::InvalidSyntax(msg) => {
                            self.err(&format!("Invalid syntax error: {}", msg))
                        }
                        ENextErr::NumericFirst() => {
                            self.err("Numeric symbols cannot be used as first in names.")
                        }
                    });
                }
            };
            // break;
        }
        Ok(self.store.clone())
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
        let allowed_chars: Vec<char> = vec![chars::UNDERLINE];
        let limited_chars: Vec<char> = vec![chars::AT, chars::AMPERSAND];
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
                    chars::QUESTION => return Ok(ENext::Question(pass)),
                    _ => {}
                };
            }
            if char.is_ascii_whitespace() || breakable.is_some() {
                if limited
                    && ((!str.starts_with(chars::AMPERSAND) && str.contains(chars::AMPERSAND))
                        || (!str.starts_with(chars::AT) && str.contains(chars::AT)))
                {
                    return Err(ENextErr::InvalidSyntax(format!(
                        "Chars {} and {} can be used only at the begging of words. Issue: {}",
                        chars::AMPERSAND,
                        chars::AT,
                        str
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

    fn err(&self, e: &str) -> String {
        let cropped = format!("{}{}", &self.content[0..self.cursor], ">>> BREAK >>>");
        let lines: Vec<&str> = cropped.split('\n').collect();
        let spaces = lines.len().to_string().len();
        for (n, l) in lines.iter().enumerate() {
            let rate = (n + 1).to_string().len();
            println!("{}{}: {}", n + 1, " ".repeat(spaces - rate), l);
        }
        String::from(e)
    }
}
