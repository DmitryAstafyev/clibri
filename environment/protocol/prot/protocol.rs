
#[derive(Debug, Clone)]
pub enum AvailableMessages {
    UserRole(UserRole),
    Identification(Identification::AvailableMessages),
    Events(Events::AvailableMessages),
    Message(Message::AvailableMessages),
    Messages(Messages::AvailableMessages),
    UserLogin(UserLogin::AvailableMessages),
    UserInfo(UserInfo::AvailableMessages),
    Users(Users::AvailableMessages),
}
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin(String),
    User(String),
    Manager(String),
    Defaults,
}
impl EnumDecode for UserRole {
    fn get_id(&self) -> u32 { 11 }
    fn extract(buf: Vec<u8>) -> Result<UserRole, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for UserRole because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::Admin(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::User(v)),
                Err(e) => Err(e)
            },
            2 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::Manager(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for UserRole")),
        }
    }
}
impl EnumEncode for UserRole {
    fn get_id(&self) -> u32 { 11 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let (buf, index) = match self {
            Self::Admin(v) => (v.encode(), 0),
            Self::User(v) => (v.encode(), 1),
            Self::Manager(v) => (v.encode(), 2),
            _ => { return Err(String::from("Not supportable option")); },
        };
        let mut buf = match buf {
            Ok(buf) => buf,
            Err(e) => { return Err(e); },
        };
        let mut buffer: Vec<u8> = vec!();
        buffer.append(&mut (index as u16).to_le_bytes().to_vec());
        buffer.append(&mut buf);
        Ok(buffer)
    }
}
impl PackingEnum for UserRole {}

pub mod Identification {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        SelfKey(SelfKey),
        SelfKeyResponse(SelfKeyResponse),
        AssignedKey(AssignedKey),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKey {
        pub uuid: Option<String>,
        pub id: Option<u64>,
        pub location: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKey {
        fn get_id() -> u32 {
            2
        }
        fn defaults() -> SelfKey {
            SelfKey {
                uuid: None,
                id: None,
                location: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(3)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.id = match Option::<u64>::get_from_storage(Source::Storage(&mut storage), Some(4)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.location = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(5)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKey {
        fn get_id(&self) -> u32 { 2 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(3)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.id.get_buf_to_store(Some(4)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.location.get_buf_to_store(Some(5)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKey { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKeyResponse {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKeyResponse {
        fn get_id() -> u32 {
            6
        }
        fn defaults() -> SelfKeyResponse {
            SelfKeyResponse {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(7)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKeyResponse {
        fn get_id(&self) -> u32 { 6 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(7)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKeyResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignedKey {
        pub uuid: Option<String>,
        pub auth: Option<bool>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for AssignedKey {
        fn get_id() -> u32 {
            8
        }
        fn defaults() -> AssignedKey {
            AssignedKey {
                uuid: None,
                auth: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(9)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.auth = match Option::<bool>::get_from_storage(Source::Storage(&mut storage), Some(10)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for AssignedKey {
        fn get_id(&self) -> u32 { 8 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(9)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.auth.get_buf_to_store(Some(10)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for AssignedKey { }

}

pub mod Events {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        UserConnected(UserConnected),
        UserDisconnected(UserDisconnected),
        Message(Message),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserConnected {
        pub username: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserConnected {
        fn get_id() -> u32 {
            13
        }
        fn defaults() -> UserConnected {
            UserConnected {
                username: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(14)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(15)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserConnected {
        fn get_id(&self) -> u32 { 13 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(14)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(15)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserConnected { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserDisconnected {
        pub username: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserDisconnected {
        fn get_id() -> u32 {
            16
        }
        fn defaults() -> UserDisconnected {
            UserDisconnected {
                username: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(17)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(18)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserDisconnected {
        fn get_id(&self) -> u32 { 16 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(17)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(18)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserDisconnected { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub timestamp: u64,
        pub user: String,
        pub message: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Message {
        fn get_id() -> u32 {
            19
        }
        fn defaults() -> Message {
            Message {
                timestamp: 0,
                user: String::from(""),
                message: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.timestamp = match u64::get_from_storage(Source::Storage(&mut storage), Some(20)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(21)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(22)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(23)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Message {
        fn get_id(&self) -> u32 { 19 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.timestamp.get_buf_to_store(Some(20)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.user.get_buf_to_store(Some(21)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(22)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(23)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Message { }

}

pub mod Message {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Accepted(Accepted),
        Denied(Denied),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub user: String,
        pub message: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            25
        }
        fn defaults() -> Request {
            Request {
                user: String::from(""),
                message: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(26)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(27)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 25 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.user.get_buf_to_store(Some(26)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(27)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            28
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(29)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 28 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(29)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Accepted { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Denied {
        pub reason: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Denied {
        fn get_id() -> u32 {
            30
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(31)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 30 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(31)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Denied { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            32
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(33)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 32 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(33)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod Messages {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Message(Message),
        Request(Request),
        Response(Response),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub timestamp: u64,
        pub user: String,
        pub uuid: String,
        pub message: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Message {
        fn get_id() -> u32 {
            35
        }
        fn defaults() -> Message {
            Message {
                timestamp: 0,
                user: String::from(""),
                uuid: String::from(""),
                message: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.timestamp = match u64::get_from_storage(Source::Storage(&mut storage), Some(36)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(37)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(38)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(39)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Message {
        fn get_id(&self) -> u32 { 35 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.timestamp.get_buf_to_store(Some(36)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.user.get_buf_to_store(Some(37)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(38)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(39)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Message { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            40
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 40 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub messages: Vec<Message>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Response {
        fn get_id() -> u32 {
            41
        }
        fn defaults() -> Response {
            Response {
                messages: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.messages = match Vec::<Message>::get_from_storage(Source::Storage(&mut storage), Some(42)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 41 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.messages.get_buf_to_store(Some(42)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Response { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            43
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(44)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 43 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(44)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserLogin {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Accepted(Accepted),
        Denied(Denied),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub username: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            46
        }
        fn defaults() -> Request {
            Request {
                username: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(47)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 46 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(47)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            48
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(49)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 48 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(49)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Accepted { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Denied {
        pub reason: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Denied {
        fn get_id() -> u32 {
            50
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(51)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 50 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(51)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Denied { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            52
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(53)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 52 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(53)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserInfo {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Accepted(Accepted),
        Denied(Denied),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            55
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 55 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub browser: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            56
        }
        fn defaults() -> Accepted {
            Accepted {
                browser: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.browser = match String::get_from_storage(Source::Storage(&mut storage), Some(57)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 56 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.browser.get_buf_to_store(Some(57)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Accepted { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Denied {
        pub reason: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Denied {
        fn get_id() -> u32 {
            58
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(59)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 58 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(59)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Denied { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            60
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(61)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 60 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(61)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod Users {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        User(User),
        Request(Request),
        Response(Response),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub name: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for User {
        fn get_id() -> u32 {
            63
        }
        fn defaults() -> User {
            User {
                name: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.name = match String::get_from_storage(Source::Storage(&mut storage), Some(64)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(65)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for User {
        fn get_id(&self) -> u32 { 63 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.name.get_buf_to_store(Some(64)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(65)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for User { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            66
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 66 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub users: Vec<User>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Response {
        fn get_id() -> u32 {
            67
        }
        fn defaults() -> Response {
            Response {
                users: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.users = match Vec::<User>::get_from_storage(Source::Storage(&mut storage), Some(68)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 67 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.users.get_buf_to_store(Some(68)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Response { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            69
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(70)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 69 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(70)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {
        match id {
            11 => match UserRole::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserRole(m)),
                Err(e) => Err(e),
            },
            2 => match Identification::SelfKey::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::SelfKey(m))),
                Err(e) => Err(e),
            },
            6 => match Identification::SelfKeyResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::SelfKeyResponse(m))),
                Err(e) => Err(e),
            },
            8 => match Identification::AssignedKey::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::AssignedKey(m))),
                Err(e) => Err(e),
            },
            13 => match Events::UserConnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::UserConnected(m))),
                Err(e) => Err(e),
            },
            16 => match Events::UserDisconnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::UserDisconnected(m))),
                Err(e) => Err(e),
            },
            19 => match Events::Message::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::Message(m))),
                Err(e) => Err(e),
            },
            25 => match Message::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            28 => match Message::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            30 => match Message::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            32 => match Message::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            35 => match Messages::Message::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Message(m))),
                Err(e) => Err(e),
            },
            40 => match Messages::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            41 => match Messages::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            43 => match Messages::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            46 => match UserLogin::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            48 => match UserLogin::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            50 => match UserLogin::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            52 => match UserLogin::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            55 => match UserInfo::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            56 => match UserInfo::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            58 => match UserInfo::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            60 => match UserInfo::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            63 => match Users::User::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::User(m))),
                Err(e) => Err(e),
            },
            66 => match Users::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            67 => match Users::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            69 => match Users::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

