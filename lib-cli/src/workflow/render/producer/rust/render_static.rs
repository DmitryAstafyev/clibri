use super::{
    helpers, helpers::render as tools, workflow::broadcast::Broadcast, workflow::request::Request,
};
use std::include_str;
use std::{
    fs,
    path::{Path, PathBuf},
};

mod paths {
    pub mod events {
        pub const connected: &str = "connected.rs";
        pub const disconnected: &str = "disconnected.rs";
        pub const error: &str = "error.rs";
        pub const dest: &str = "events";
    }
    pub mod consumer {
        pub const identification: &str = "identification.rs";
        pub const module: &str = "mod.rs";
        pub const dest: &str = "implementation/consumer";
    }
    pub mod emitters {
        pub const connected: &str = "connected.rs";
        pub const disconnected: &str = "disconnected.rs";
        pub const error: &str = "error.rs";
        pub const dest: &str = "implementation/emitters";
    }
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

    pub fn render(&self, base: &Path) -> Result<(), String> {
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::connected)?,
            include_str!("./static/events/connected.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::disconnected)?,
            include_str!("./static/events/disconnected.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::error)?,
            include_str!("./static/events/error.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::identification)?,
            include_str!("./static/implementation/consumer/identification.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::module)?,
            include_str!("./static/implementation/consumer/mod.rs").to_owned(),
            true,
        )?;

        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::connected)?,
            include_str!("./static/implementation/emitters/connected.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::disconnected)?,
            include_str!("./static/implementation/emitters/disconnected.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::error)?,
            include_str!("./static/implementation/emitters/error.rs").to_owned(),
            true,
        )?;
        Ok(())
    }

    fn get_dest_file(&self, base: &Path, path: &str, file_name: &str) -> Result<PathBuf, String> {
        let dest = base.join(path);
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(file_name))
    }
}
