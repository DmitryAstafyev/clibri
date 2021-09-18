use super::implementation::protocol;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Context {
    users: HashMap<Uuid, protocol::Users::User>,
    messages: HashMap<Uuid, protocol::Messages::Message>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: HashMap::new(),
        }
    }

    pub fn timestamp() -> Result<u64, String> {
        let start = SystemTime::now();
        Ok(start
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis() as u64)
    }

    pub async fn is_user_exist(&self, username: &str) -> bool {
        self.users
            .iter()
            .find_map(|(uuid, val)| {
                if val.name == username {
                    Some(uuid)
                } else {
                    None
                }
            })
            .is_some()
    }

    pub async fn add_user(&mut self, uuid: Uuid, username: &str) {
        self.users.insert(
            uuid,
            protocol::Users::User {
                name: username.to_owned(),
                uuid: uuid.to_string(),
            },
        );
    }

    pub async fn remove_user(&mut self, uuid: Uuid) -> Option<protocol::Users::User> {
        self.users.remove(&uuid)
    }

    pub fn add_message(
        &mut self,
        username: &str,
        message: String,
    ) -> Result<protocol::Events::Message, String> {
        let uuid = Uuid::new_v4();
        let msg = protocol::Messages::Message {
            timestamp: Context::timestamp()?,
            uuid: uuid.to_string(),
            user: username.to_string(),
            message,
        };
        self.messages.insert(uuid, msg.clone());
        Ok(protocol::Events::Message {
            timestamp: msg.timestamp,
            uuid: msg.uuid,
            user: msg.user,
            message: msg.message,
        })
    }

    pub fn get_messages(&self) -> Vec<protocol::Messages::Message> {
        self.messages.values().cloned().collect()
    }

    pub fn get_users(&self) -> Vec<protocol::Users::User> {
        self.users.values().cloned().collect()
    }
}
