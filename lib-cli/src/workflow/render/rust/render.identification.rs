use super::{
    helpers,
    helpers::{
        render as tools,
    },
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
[[self_key_overwrite]]
        }
    }

    pub fn assign(&mut self, key: Protocol::[[assigned_key]], overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(key);
        } else if let Some(existing) = &mut self.assigned {
[[assign_key_overwrite]]
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
    pub const KEY_OPT_FIELD_CHECK: &str =
r#"if let Some([[field]]) = key.[[field]] {
    existing.[[field]] = Some([[field]]);
}"#;
    pub const KEY_FIELD_CHECK: &str = "existing.[[field]] = key.[[field]]";
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
        protocol: &Protocol,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[self_key]]", &self.into_rust_path(&config.get_self()?));
        output = output.replace("[[assigned_key]]", &self.into_rust_path(&config.get_assigned()?));
        output = output.replace("[[self_key_overwrite]]", &tools::inject_tabs(3, self.self_key_overwrite(config, protocol)?));
        output = output.replace("[[assign_key_overwrite]]", &tools::inject_tabs(3, self.assign_key_overwrite(config, protocol)?));
        helpers::fs::write(dest, output, true)
    }

    fn self_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_self()?) {
            let mut output: String = String::new();
            for (pos, field) in strct.fields.iter().enumerate() {
                output = format!("{}{}",
                    output,
                    (if field.optional {
                        templates::KEY_OPT_FIELD_CHECK
                    } else {
                        templates::KEY_FIELD_CHECK
                    }).replace(
                        "[[field]]",
                        &field.name,
                    )
                );
                if pos < strct.fields.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
    }

    fn assign_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_assigned()?) {
            let mut output: String = String::new();
            for (pos, field) in strct.fields.iter().enumerate() {
                output = format!("{}{}",
                    output,
                    (if field.optional {
                        templates::KEY_OPT_FIELD_CHECK
                    } else {
                        templates::KEY_FIELD_CHECK
                    }).replace(
                        "[[field]]",
                        &field.name,
                    )
                );
                if pos < strct.fields.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
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

