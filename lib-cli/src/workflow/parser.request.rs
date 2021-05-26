use super::{Target, ENext, EntityParser};

#[derive(Debug, Clone)]
pub struct Conslution {
    pub name: String,
    pub broadcast: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request: Option<String>,
    pub response: Option<String>,
    pub err: Option<String>,
    pub conclusions: Vec<Conslution>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            request: None,
            response: None,
            err: None,
            conclusions: vec![],
        }
    }
}

impl EntityParser for Request {
    
    fn open(word: String) -> Option<Self> {
        Some(Request::new())
    }

    fn next(&mut self, entity: ENext) -> Result<usize, String> {
        Err(String::from(""))
    }

    fn closed(&self) -> bool {
        true
    }

    fn print(&self) {

    }

}