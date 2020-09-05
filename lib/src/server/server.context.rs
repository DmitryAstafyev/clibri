use super:: { connection };
use connection:: { Connection };
use std::sync::{ Arc, RwLock };
use std::collections::{ HashMap };

#[derive(Clone)]
pub struct Context {
    pub uuid: String,
    pub connections: Arc<RwLock<HashMap<String, Connection>>>,
}

impl Context {
    
    #[allow(dead_code)]
    pub fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let uuid = self.uuid.clone();
        self.send_to(uuid, buffer)
    }

    #[allow(dead_code)]
    pub fn send_to(&mut self, uuid: String, buffer: Vec<u8>) -> Result<(), String> {
        match self.connections.write() {
            Ok(mut connections) => {
                if let Some(connection) = connections.get_mut(&uuid.clone()) {
                    return connection.send(buffer);
                } else {
                    return Err("Fail to get access to connection".to_string());
                }
            },
            Err(e) => Err(format!("Fail to get access to connections due error: {}", e))
        }
    }

    pub fn get_uuid(&mut self) -> String {
        self.uuid.clone()
    }

}