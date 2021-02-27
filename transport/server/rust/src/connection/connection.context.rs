use super:: { connection };
use connection:: { Connection };
use std::sync::{ Arc, RwLock };
use std::collections::{ HashMap };
use uuid::Uuid;

#[derive(Clone)]
pub struct ConnectionContext {
    pub uuid: Uuid,
    pub connections: Arc<RwLock<HashMap<Uuid, Connection>>>,
}

impl ConnectionContext {
    
    #[allow(dead_code)]
    pub fn send_buffer(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let uuid = self.uuid.clone();
        self.send_buffer_to(uuid, buffer)
    }

    #[allow(dead_code)]
    pub fn send_buffer_to(&mut self, uuid: Uuid, buffer: Vec<u8>) -> Result<(), String> {
        match self.connections.write() {
            Ok(mut connections) => {
                if let Some(connection) = connections.get_mut(&uuid) {
                    connection.send(buffer)
                } else {
                    Err("Fail to get access to connection".to_string())
                }
            },
            Err(e) => Err(format!("Fail to get access to connections due error: {}", e))
        }
    }

    pub fn get_uuid(&mut self) -> Uuid {
        self.uuid.clone()
    }

}
/*
impl ServerConnectionContext for ConnectionContext {

    #[allow(dead_code)]
    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let uuid = self.uuid.clone();
        self.send_to(uuid, buffer)
    }

    #[allow(dead_code)]
    fn send_to(&mut self, uuid: Uuid, buffer: Vec<u8>) -> Result<(), String> {
        match self.connections.write() {
            Ok(mut connections) => {
                if let Some(connection) = connections.get_mut(&uuid) {
                    connection.send(buffer)
                } else {
                    Err("Fail to get access to connection".to_string())
                }
            },
            Err(e) => Err(format!("Fail to get access to connections due error: {}", e))
        }
    }

*/