use super::helpers;
use std::{
    fs, include_str,
    path::{Path, PathBuf},
};
use toml::{map::Map, Value};

pub struct Render {}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, base: &Path) -> Result<(), String> {
        let target = self.get_target_file(base)?;
        let content = fs::read_to_string(target.clone())
            .map_err(|e| format!("Fail read {}; error: {}", target.to_string_lossy(), e))?;
        let cargo = content
            .parse::<Value>()
            .map_err(|e| format!("Fail parse {}; error: {}", target.to_string_lossy(), e))?;
        let mut cargo_original = cargo.clone();
        let mut deps = match cargo {
            Value::Table(mut cargo) => {
                if let Some(deps) = cargo.remove("dependencies") {
                    deps
                } else {
                    Value::Table(Map::new())
                }
            }
            _ => Value::Table(Map::new()),
        };
        let required = match include_str!("./static/required.toml")
            .to_owned()
            .parse::<Value>()
            .map_err(|e| format!("Fail parse required.toml; error: {}", e))?
        {
            Value::Table(required) => required,
            _ => {
                return Err(String::from("Fail parse required.toml"));
            }
        };
        for (key, value) in required {
            deps = match deps {
                Value::Table(mut deps) => {
                    deps.insert(key.clone(), value);
                    println!("[DEP] added dependency\"{}\"", key);
                    Value::Table(deps)
                }
                _ => deps,
            };
        }
        cargo_original = match cargo_original {
            Value::Table(mut cargo) => {
                cargo.insert(String::from("dependencies"), deps);
                Value::Table(cargo)
            }
            _ => cargo_original,
        };
        helpers::fs::write(
            target.clone(),
            toml::ser::to_string_pretty(&cargo_original)
                .map_err(|e| format!("Fail to write {}; error: {}", target.to_string_lossy(), e))?,
            true,
        )
    }

    fn get_target_file(&self, base: &Path) -> Result<PathBuf, String> {
        let mut current = base.to_path_buf();
        while let Some(parent) = current.parent() {
            let target = current.join("Cargo.toml");
            if target.exists() {
                return Ok(target);
            } else {
                current = parent.to_path_buf();
            }
        }
        Err(format!(
            "Cannot find Cargo.toml. Checked all nested starting from {}",
            base.to_string_lossy()
        ))
    }
}
