use super::helpers;
use serde_json::{map::Map, Value};
use std::{
    fs, include_str,
    path::{Path, PathBuf},
};

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
        let package = content
            .parse::<Value>()
            .map_err(|e| format!("Fail parse {}; error: {}", target.to_string_lossy(), e))?;
        let mut package_original = package.clone();
        let mut deps = match package {
            Value::Object(mut package) => {
                if let Some(deps) = package.remove("dependencies") {
                    deps
                } else {
                    Value::Object(Map::new())
                }
            }
            _ => Value::Object(Map::new()),
        };
        let required = match include_str!("./static/required.json")
            .to_owned()
            .parse::<Value>()
            .map_err(|e| format!("Fail parse required.toml; error: {}", e))?
        {
            Value::Object(required) => required,
            _ => {
                return Err(String::from("Fail parse required.toml"));
            }
        };
        for (key, value) in required {
            deps = match deps {
                Value::Object(mut deps) => {
                    deps.insert(key.clone(), value);
                    println!("[DEP] added dependency\"{}\"", key);
                    Value::Object(deps)
                }
                _ => deps,
            };
        }
        package_original = match package_original {
            Value::Object(mut package) => {
                package.insert(String::from("dependencies"), deps);
                Value::Object(package)
            }
            _ => package_original,
        };
        helpers::fs::write(
            target.clone(),
            serde_json::to_string_pretty(&package_original)
                .map_err(|e| format!("Fail to write {}; error: {}", target.to_string_lossy(), e))?,
            true,
        )
    }

    fn get_target_file(&self, base: &Path) -> Result<PathBuf, String> {
        let mut current = base.to_path_buf();
        while let Some(parent) = current.parent() {
            let target = current.join("package.json");
            if target.exists() {
                return Ok(target);
            } else {
                current = parent.to_path_buf();
            }
        }
        Err(format!(
            "Cannot find package.json. Checked all nested starting from {}",
            base.to_string_lossy()
        ))
    }
}
