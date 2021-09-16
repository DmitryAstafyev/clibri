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

    pub fn timestamp() -> Result<Duration, String> {
        let start = SystemTime::now();
        start.duration_since(UNIX_EPOCH).map_err(|e| e.to_string())
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

    pub async fn add_user(&mut self, username: &str) -> Uuid {
        let uuid = Uuid::new_v4();
        self.users.insert(
            uuid,
            protocol::Users::User {
                name: username.to_owned(),
                uuid: uuid.to_string(),
            },
        );
        uuid
    }
}
