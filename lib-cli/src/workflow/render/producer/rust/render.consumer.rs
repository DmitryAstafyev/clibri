use super::{
    helpers,
    workflow::{
        config::{
            Config
        }
    },
    Protocol,
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

mod templates {
    pub const MODULE: &str = r#"
use super::consumer_identification::{Filter, Identification};
use super::{tools, ConsumersChannel, Protocol};
use fiber::logger::Logger;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;
pub struct Cx {
    uuid: Uuid,
    consumers: UnboundedSender<ConsumersChannel>,
}

impl Cx {
    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        if let Err(e) = self
            .consumers
            .send(ConsumersChannel::SendTo((self.uuid, buffer)))
        {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    pub fn send_to(&self, buffer: Vec<u8>, filter: Filter) -> Result<(), String> {
        if let Err(e) = self
            .consumers
            .send(ConsumersChannel::SendByFilter((filter, buffer)))
        {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn assign(
        &self,
        assigned: Protocol::[[assigned_key]],
        overwrite: bool,
    ) -> Result<(), String> {
        if let Err(e) = self.consumers.send(ConsumersChannel::Assign((
            self.uuid,
            assigned,
            overwrite,
        ))) {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}

pub struct Consumer {
    uuid: Uuid,
    buffer: Protocol::Buffer<Protocol::AvailableMessages>,
    identification: Identification,
    cx: Cx,
    sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
}

impl Consumer {
    pub fn new(
        uuid: Uuid,
        consumers: UnboundedSender<ConsumersChannel>,
        sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    ) -> Self {
        Consumer {
            uuid,
            buffer: Protocol::Buffer::new(),
            identification: Identification::new(uuid),
            cx: Cx { uuid, consumers },
            sender,
        }
    }

    pub fn chunk(&mut self, buffer: &Vec<u8>) -> Result<(), String> {
        if let Err(e) = self.buffer.chunk(buffer, Some(self.uuid.to_string())) {
            Err(format!("{:?}", e))
        } else {
            Ok(())
        }
    }

    pub fn next(&mut self) -> Option<(Protocol::AvailableMessages, Protocol::PackageHeader)> {
        if let Some(msg) = self.buffer.next() {
            Some((msg.msg, msg.header))
        } else {
            None
        }
    }

    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        let len = buffer.len();
        if let Err(e) = self.sender.send((buffer, Some(self.uuid))) {
            Err(tools::logger.err(&format!(
                "{}:: Fail to send buffer {} bytes, due error: {}",
                self.get_uuid(),
                len,
                e
            )))
        } else {
            tools::logger.debug(&format!(
                "{}:: Has been sent a buffer {} bytes",
                self.get_uuid(),
                len
            ));
            Ok(())
        }
    }

    pub fn send_if(&self, buffer: Vec<u8>, filter: Filter) -> Result<bool, String> {
        if self.identification.filter(filter) {
            if let Err(e) = self.send(buffer) {
                Err(e)
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub fn is_filtered(&self, filter: Filter) -> bool {
        self.identification.filter(filter)
    }

    pub fn get_cx(&mut self) -> &Cx {
        &self.cx
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn key(&mut self, key: Protocol::[[self_key]], overwrite: bool) -> String {
        self.identification.key(key, overwrite);
        self.uuid.to_string()
    }

    pub fn assign(&mut self, key: Protocol::[[assigned_key]], overwrite: bool) {
        self.identification.assign(key, overwrite);
    }

    pub fn assigned(&self) -> bool {
        self.identification.assigned()
    }
}
"#;
}

pub struct RenderConsumer {
}

impl Default for RenderConsumer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderConsumer {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        config: &Config,
        _protocol: &Protocol,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[self_key]]", &self.into_rust_path(&config.get_self()?));
        output = output.replace("[[assigned_key]]", &self.into_rust_path(&config.get_assigned()?));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("consumer");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("consumer.rs"))
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }



}

