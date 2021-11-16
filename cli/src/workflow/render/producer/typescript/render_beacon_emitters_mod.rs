use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import * as Protocol from "../../implementation/protocol";

[[handlers]]

export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { Protocol };"#;
}

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

    pub fn render(&self, base: &Path, beacons: &[Broadcast]) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut mods = String::new();
        for beacon in beacons.iter() {
            mods = format!(
                "{}export {{ handler as {}Handler }} from \"./{}\";\n",
                mods,
                helpers::string::first_letter_lowercase(&beacon.reference.replace(".", "")),
                beacon.reference.to_lowercase()
            );
        }
        let output = templates::MODULE.to_owned().replace("[[handlers]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("beacons");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("index.ts"))
    }
}
