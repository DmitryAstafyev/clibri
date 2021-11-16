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
        pub const ready: &str = "ready.rs";
        pub const shutdown: &str = "shutdown.rs";
        pub const dest: &str = "events";
    }
    pub mod consumer {
        pub const module: &str = "mod.rs";
        pub const dest: &str = "implementation/consumer";
    }
    pub mod emitters {
        pub const connected: &str = "connected.rs";
        pub const disconnected: &str = "disconnected.rs";
        pub const error: &str = "error.rs";
        pub const ready: &str = "ready.rs";
        pub const shutdown: &str = "shutdown.rs";
        pub const dest: &str = "implementation/emitters";
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

    pub fn render(&self, base: &Path, events: &[Event]) -> Result<(), String> {
        if events
            .iter()
            .find(|event| match event.get_reference() {
                Ok(reference) => reference == "connected",
                Err(_) => false,
            })
            .is_none()
        {
            helpers::fs::write(
                self.get_dest_file(base, paths::events::dest, paths::events::connected)?,
                include_str!("./static/events/connected.rs").to_owned(),
                true,
            )?;
            helpers::fs::write(
                self.get_dest_file(base, paths::emitters::dest, paths::emitters::connected)?,
                include_str!("./static/implementation/emitters/connected.rs").to_owned(),
                true,
            )?;
        }
        if events
            .iter()
            .find(|event| match event.get_reference() {
                Ok(reference) => reference == "disconnected",
                Err(_) => false,
            })
            .is_none()
        {
            helpers::fs::write(
                self.get_dest_file(base, paths::events::dest, paths::events::disconnected)?,
                include_str!("./static/events/disconnected.rs").to_owned(),
                true,
            )?;
            helpers::fs::write(
                self.get_dest_file(base, paths::emitters::dest, paths::emitters::disconnected)?,
                include_str!("./static/implementation/emitters/disconnected.rs").to_owned(),
                true,
            )?;
        }
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::error)?,
            include_str!("./static/events/error.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::ready)?,
            include_str!("./static/events/ready.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::shutdown)?,
            include_str!("./static/events/shutdown.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::module)?,
            include_str!("./static/implementation/consumer/mod.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::error)?,
            include_str!("./static/implementation/emitters/error.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::ready)?,
            include_str!("./static/implementation/emitters/ready.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::shutdown)?,
            include_str!("./static/implementation/emitters/shutdown.rs").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::module::dest, paths::module::module)?,
            include_str!("./static/mod.rs").to_owned(),
            true,
        )?;
        self.create_if(
            base,
            paths::context::dest,
            paths::context::module,
            include_str!("./static/context.rs").to_owned(),
        )?;
        Ok(())
    }

    fn create_if(
        &self,
        base: &Path,
        path: &str,
        file_name: &str,
        content: String,
    ) -> Result<(), String> {
        let dest = self.get_dest_file(base, path, file_name)?;
        if !dest.exists() {
            helpers::fs::write(self.get_dest_file(base, path, file_name)?, content, true)?;
        } else {
            println!("[SKIP]: {}", dest.to_string_lossy());
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
