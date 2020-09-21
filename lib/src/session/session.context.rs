use super:: { connection, msg_outgoing_builder };
use connection:: { Connection };
use std::sync::{ Arc, RwLock };
use std::collections::{ HashMap };
use msg_outgoing_builder::Message as OutgoingMessage;

#[derive(Clone)]
pub struct SessionContext {
    pub uuid: String,
    pub connections: Arc<RwLock<HashMap<String, Connection>>>,
}

impl SessionContext {
    
    #[allow(dead_code)]
    pub fn send_buffer(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let uuid = self.uuid.clone();
        self.send_buffer_to(uuid, buffer)
    }

    #[allow(dead_code)]
    pub fn send_buffer_to(&mut self, uuid: String, buffer: Vec<u8>) -> Result<(), String> {
        match self.connections.write() {
            Ok(mut connections) => {
                if let Some(connection) = connections.get_mut(&uuid.clone()) {
                    return connection.buffer(buffer);
                } else {
                    return Err("Fail to get access to connection".to_string());
                }
            },
            Err(e) => Err(format!("Fail to get access to connections due error: {}", e))
        }
    }

    #[allow(dead_code)]
    pub fn send_msg(&mut self, msg: impl OutgoingMessage) -> Result<(), String> {
        let uuid = self.uuid.clone();
        self.send_msg_to(uuid, msg)
    }

    #[allow(dead_code)]
    pub fn send_msg_to(&mut self, uuid: String, msg: impl OutgoingMessage) -> Result<(), String> {
        match self.connections.write() {
            Ok(mut connections) => {
                if let Some(connection) = connections.get_mut(&uuid.clone()) {
                    return connection.msg(msg);
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