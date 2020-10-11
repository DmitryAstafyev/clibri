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
        let content: String = match self.get_content(self._src.clone()) {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let errs: Vec<String> = vec![];
        for char in content.chars() {

        }
        Ok(())
    }

    fn next(chars: Chars) -> Result<ENext, ENextErr> {
        let mut str: String = String::new();
        let mut pos: usize = 0;
        let break_chars: Vec<char> = vec![';', '{', '}'];
        let allowed_chars: Vec<char> = vec!['_'];
        for char in chars {
            pos += 1;
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
                    ';' => return Ok(ENext::Semicolon(pos)),
                    '{' => return Ok(ENext::OpenStruct(pos)),
                    '}' => return Ok(ENext::CloseStruct(pos)),
                    _ => {}
                };
            }
            if char.is_ascii_whitespace() || breakable {
                return Ok(ENext::Word((str, pos)));
            }
            let allowed: bool = allowed_chars.iter().any(|&c| c == char);
            if !char.is_ascii_alphanumeric() || !allowed {
                return Err(ENextErr::NotSupported(format!("found not supportable char: {}", char)))
            }
            str.push(char);
        }
        Ok(ENext::Word((str, pos)))
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