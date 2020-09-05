use serde_json::{Result as ResultJSON};

pub trait Extractor<'de> {
    
    type Msg: serde::Serialize + serde::Deserialize<'de>;

    fn new(str: &'de str) -> Result<Self::Msg, String> {
        match serde_json::from_str(str) as ResultJSON<Self::Msg> {
            Ok(msg) => Ok(msg),
            Err(e) => Err(e.to_string()),
        }
    }

}