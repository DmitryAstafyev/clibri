use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        config::{
            Config
        }
    }
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
use super::{
    tools,
    Protocol
};
use fiber::logger::Logger;
use std::{
    cmp::{
        Eq,
        PartialEq
    },
    sync::Arc,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Equal,
    NotEqual,
}
pub type FilterCallback = dyn Fn(
        Uuid,
        Option<Protocol::[[self_key]]>,
        Option<Protocol::[[assigned_key]]>,
    ) -> bool
    + Send
    + Sync;

#[derive(Clone)]
pub struct Filter {
    pub uuid: Option<(Uuid, Condition)>,
    pub assign: Option<bool>,
    pub filter: Option<Arc<Box<FilterCallback>>>,
}

#[derive(Debug, Clone)]
pub struct Identification {
    uuid: Uuid,
    key: Option<Protocol::[[self_key]]>,
    assigned: Option<Protocol::[[assigned_key]]>,
}

impl Identification {
    pub fn new(uuid: Uuid) -> Self {
        Identification {
            uuid,
            key: None,
            assigned: None,
        }
    }

    pub fn key(&mut self, key: Protocol::[[self_key]], overwrite: bool) {
        if overwrite || self.key.is_none() {
            self.key = Some(key);
        } else if let Some(existing) = &mut self.key {
            if let Some(uuid) = key.uuid {
                existing.uuid = Some(uuid);
            }
            if let Some(id) = key.id {
                existing.id = Some(id);
            }
            if let Some(location) = key.location {
                existing.location = Some(location);
            }
        }
    }

    pub fn assign(&mut self, assigned: Protocol::[[assigned_key]], overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(assigned);
        } else if let Some(existing) = &mut self.assigned {
            if let Some(uuid) = assigned.uuid {
                existing.uuid = Some(uuid);
            }
            if let Some(auth) = assigned.auth {
                existing.auth = Some(auth);
            }
        }
    }

    pub fn filter(&self, filter: Filter) -> bool {
        if let Some((uuid, condition)) = filter.uuid {
            return match condition {
                Condition::Equal => uuid == self.uuid,
                Condition::NotEqual => uuid != self.uuid,
            };
        }
        if let Some(assign) = filter.assign {
            return assign == self.assigned();
        }
        if let Some(filter) = filter.filter {
            return filter(self.uuid, self.key.clone(), self.assigned.clone());
        }
        false
    }

    pub fn assigned(&self) -> bool {
        if self.assigned.is_none() {
            tools::logger.warn("Client doesn't have producer identification");
        }
        self.key.is_some()
    }
}
"#;
}

pub struct RenderIdentification {
}

impl Default for RenderIdentification {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderIdentification {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        config: &Config,
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
        Ok(dest.join("consumer_identification.rs"))
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }



}

