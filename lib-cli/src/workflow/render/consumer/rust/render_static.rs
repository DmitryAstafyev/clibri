use super::{helpers, workflow::event::Event};
use std::include_str;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[allow(non_upper_case_globals)]
mod paths {
    pub mod events {
        pub const connected: &str = "connected.rs";
        pub const disconnected: &str = "disconnected.rs";
        pub const error: &str = "error.rs";
        pub const reconnect: &str = "reconnect.rs";
        pub const shutdown: &str = "shutdown.rs";
        pub const module: &str = "mod.rs";
        pub const dest: &str = "events";
    }
    pub mod consumer {
        pub const api: &str = "api.rs";
        pub const error: &str = "error.rs";
        pub const options: &str = "options.rs";
        pub const dest: &str = "implementation/consumer";
    }
    pub mod implementation {
        pub const module: &str = "mod.rs";
        pub const dest: &str = "implementation";
    }
    pub mod module {
        pub const module: &str = "mod.rs";
        pub const dest: &str = "";
    }
    pub mod context {
        pub const module: &str = "context.rs";
        pub const dest: &str = "";
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
            self.get_dest_file(base, paths::events::dest, paths::events::module)?,
            include_str!("./static/events/mod.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::reconnect)?,
            include_str!("./static/events/reconnect.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::shutdown)?,
            include_str!("./static/events/shutdown.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::api)?,
            include_str!("./static/implementation/consumer/api.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::error)?,
            include_str!("./static/implementation/consumer/error.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::options)?,
            include_str!("./static/implementation/consumer/options.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(
                base,
                paths::implementation::dest,
                paths::implementation::module,
            )?,
            include_str!("./static/implementation/mod.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::module::dest, paths::module::module)?,
            include_str!("./static/mod.rs").to_owned(),
            true,
        )?;
        let context_dest =
            self.get_dest_file(base, paths::context::dest, paths::context::module)?;
        if !context_dest.exists() {
            helpers::fs::write(
                self.get_dest_file(base, paths::context::dest, paths::context::module)?,
                include_str!("./static/context.rs").to_owned(),
                true,
            )?;
        }
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
