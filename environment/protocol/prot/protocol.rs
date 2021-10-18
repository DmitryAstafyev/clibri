
#[derive(Debug, Clone)]
pub enum AvailableMessages {
    UserRole(UserRole),
    Identification(Identification::AvailableMessages),
    Events(Events::AvailableMessages),
    Beacons(Beacons::AvailableMessages),
    ServerEvents(ServerEvents::AvailableMessages),
    Message(Message::AvailableMessages),
    Messages(Messages::AvailableMessages),
    UserLogin(UserLogin::AvailableMessages),
    UserInfo(UserInfo::AvailableMessages),
    Users(Users::AvailableMessages),
    InternalServiceGroup(InternalServiceGroup::AvailableMessages),
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

pub mod Beacons {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        LikeUser(LikeUser),
        LikeMessage(LikeMessage),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LikeUser {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for LikeUser {
        fn get_id() -> u32 {
            25
        }
        fn defaults() -> LikeUser {
            LikeUser {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(26)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for LikeUser {
        fn get_id(&self) -> u32 { 25 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(26)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for LikeUser { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LikeMessage {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for LikeMessage {
        fn get_id() -> u32 {
            27
        }
        fn defaults() -> LikeMessage {
            LikeMessage {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(28)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for LikeMessage {
        fn get_id(&self) -> u32 { 27 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(28)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for LikeMessage { }

}

pub mod ServerEvents {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        UserKickOff(UserKickOff),
        UserAlert(UserAlert),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserKickOff {
        pub reason: Option<String>,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserKickOff {
        fn get_id() -> u32 {
            30
        }
        fn defaults() -> UserKickOff {
            UserKickOff {
                reason: None,
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(31)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(32)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserKickOff {
        fn get_id(&self) -> u32 { 30 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(31)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(32)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserKickOff { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserAlert {
        pub reason: Option<String>,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserAlert {
        fn get_id() -> u32 {
            33
        }
        fn defaults() -> UserAlert {
            UserAlert {
                reason: None,
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(34)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(35)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserAlert {
        fn get_id(&self) -> u32 { 33 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(34)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(35)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserAlert { }

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
            37
        }
        fn defaults() -> Request {
            Request {
                user: String::from(""),
                message: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(38)) {
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
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 37 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.user.get_buf_to_store(Some(38)) {
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
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            40
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(41)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 40 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(41)) {
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
            42
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(43)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 42 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(43)) {
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
            44
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(45)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 44 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(45)) {
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
            47
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
            self.timestamp = match u64::get_from_storage(Source::Storage(&mut storage), Some(48)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(49)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(50)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(51)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Message {
        fn get_id(&self) -> u32 { 47 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.timestamp.get_buf_to_store(Some(48)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.user.get_buf_to_store(Some(49)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(50)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(51)) {
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
            52
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
        fn get_id(&self) -> u32 { 52 }
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
            53
        }
        fn defaults() -> Response {
            Response {
                messages: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.messages = match Vec::<Message>::get_from_storage(Source::Storage(&mut storage), Some(54)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 53 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.messages.get_buf_to_store(Some(54)) {
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
            55
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(56)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 55 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(56)) {
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
            58
        }
        fn defaults() -> Request {
            Request {
                username: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(59)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 58 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(59)) {
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
            60
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(61)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 60 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(61)) {
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
            62
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(63)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 62 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(63)) {
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
            64
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(65)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 64 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(65)) {
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
            67
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
        fn get_id(&self) -> u32 { 67 }
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
            68
        }
        fn defaults() -> Accepted {
            Accepted {
                browser: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.browser = match String::get_from_storage(Source::Storage(&mut storage), Some(69)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 68 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.browser.get_buf_to_store(Some(69)) {
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
            70
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(71)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 70 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(71)) {
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
            72
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(73)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 72 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(73)) {
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
            75
        }
        fn defaults() -> User {
            User {
                name: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.name = match String::get_from_storage(Source::Storage(&mut storage), Some(76)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(77)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for User {
        fn get_id(&self) -> u32 { 75 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.name.get_buf_to_store(Some(76)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(77)) {
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
            78
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
        fn get_id(&self) -> u32 { 78 }
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
            79
        }
        fn defaults() -> Response {
            Response {
                users: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.users = match Vec::<User>::get_from_storage(Source::Storage(&mut storage), Some(80)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 79 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.users.get_buf_to_store(Some(80)) {
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
            81
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(82)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 81 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(82)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod InternalServiceGroup {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        SelfKeyResponse(SelfKeyResponse),
        HashRequest(HashRequest),
        HashResponse(HashResponse),
        BeaconConfirmation(BeaconConfirmation),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKeyResponse {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKeyResponse {
        fn get_id() -> u32 {
            84
        }
        fn defaults() -> SelfKeyResponse {
            SelfKeyResponse {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(85)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKeyResponse {
        fn get_id(&self) -> u32 { 84 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(85)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKeyResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HashRequest {
        pub protocol: String,
        pub workflow: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for HashRequest {
        fn get_id() -> u32 {
            86
        }
        fn defaults() -> HashRequest {
            HashRequest {
                protocol: String::from(""),
                workflow: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.protocol = match String::get_from_storage(Source::Storage(&mut storage), Some(87)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.workflow = match String::get_from_storage(Source::Storage(&mut storage), Some(88)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashRequest {
        fn get_id(&self) -> u32 { 86 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.protocol.get_buf_to_store(Some(87)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.workflow.get_buf_to_store(Some(88)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for HashRequest { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HashResponse {
        pub error: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for HashResponse {
        fn get_id() -> u32 {
            89
        }
        fn defaults() -> HashResponse {
            HashResponse {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(90)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashResponse {
        fn get_id(&self) -> u32 { 89 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(90)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for HashResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BeaconConfirmation {
        pub error: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for BeaconConfirmation {
        fn get_id() -> u32 {
            91
        }
        fn defaults() -> BeaconConfirmation {
            BeaconConfirmation {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(92)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for BeaconConfirmation {
        fn get_id(&self) -> u32 { 91 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(92)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for BeaconConfirmation { }

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
            25 => match Beacons::LikeUser::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::LikeUser(m))),
                Err(e) => Err(e),
            },
            27 => match Beacons::LikeMessage::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::LikeMessage(m))),
                Err(e) => Err(e),
            },
            30 => match ServerEvents::UserKickOff::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::ServerEvents(ServerEvents::AvailableMessages::UserKickOff(m))),
                Err(e) => Err(e),
            },
            33 => match ServerEvents::UserAlert::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::ServerEvents(ServerEvents::AvailableMessages::UserAlert(m))),
                Err(e) => Err(e),
            },
            37 => match Message::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            40 => match Message::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            42 => match Message::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            44 => match Message::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            47 => match Messages::Message::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Message(m))),
                Err(e) => Err(e),
            },
            52 => match Messages::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            53 => match Messages::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            55 => match Messages::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            58 => match UserLogin::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            60 => match UserLogin::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            62 => match UserLogin::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            64 => match UserLogin::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            67 => match UserInfo::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            68 => match UserInfo::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            70 => match UserInfo::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            72 => match UserInfo::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            75 => match Users::User::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::User(m))),
                Err(e) => Err(e),
            },
            78 => match Users::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            79 => match Users::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            81 => match Users::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            84 => match InternalServiceGroup::SelfKeyResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::SelfKeyResponse(m))),
                Err(e) => Err(e),
            },
            86 => match InternalServiceGroup::HashRequest::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashRequest(m))),
                Err(e) => Err(e),
            },
            89 => match InternalServiceGroup::HashResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashResponse(m))),
                Err(e) => Err(e),
            },
            91 => match InternalServiceGroup::BeaconConfirmation::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::BeaconConfirmation(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

pub fn hash() -> String { String::from("F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE") }
