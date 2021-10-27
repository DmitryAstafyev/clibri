use super::{helpers, workflow::event::Event};
use std::include_str;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[allow(non_upper_case_globals)]
mod paths {
    pub mod events {
        pub const connected: &str = "connected.ts";
        pub const disconnected: &str = "disconnected.ts";
        pub const error: &str = "error.ts";
        pub const ready: &str = "ready.ts";
        pub const shutdown: &str = "shutdown.ts";
        pub const dest: &str = "events";
    }
    pub mod consumer {
        pub const module: &str = "index.ts";
        pub const filter: &str = "filter.ts";
        pub const dest: &str = "implementation/consumer";
    }
    pub mod emitters {
        pub const connected: &str = "connected.ts";
        pub const disconnected: &str = "disconnected.ts";
        pub const error: &str = "error.ts";
        pub const ready: &str = "ready.ts";
        pub const shutdown: &str = "shutdown.ts";
        pub const dest: &str = "implementation/events";
    }
    pub mod index {
        pub const module: &str = "index.ts";
        pub const dest: &str = "";
    }
    pub mod context {
        pub const module: &str = "context.ts";
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
                include_str!("./static/events/connected.ts").to_owned(),
                true,
            )?;
            helpers::fs::write(
                self.get_dest_file(base, paths::emitters::dest, paths::emitters::connected)?,
                include_str!("./static/implementation/events/connected.ts").to_owned(),
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
                include_str!("./static/events/disconnected.ts").to_owned(),
                true,
            )?;
            helpers::fs::write(
                self.get_dest_file(base, paths::emitters::dest, paths::emitters::disconnected)?,
                include_str!("./static/implementation/events/disconnected.ts").to_owned(),
                true,
            )?;
        }
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::error)?,
            include_str!("./static/events/error.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::ready)?,
            include_str!("./static/events/ready.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::events::dest, paths::events::shutdown)?,
            include_str!("./static/events/shutdown.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::module)?,
            include_str!("./static/implementation/consumer/index.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::consumer::dest, paths::consumer::filter)?,
            include_str!("./static/implementation/consumer/filter.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::error)?,
            include_str!("./static/implementation/events/error.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::ready)?,
            include_str!("./static/implementation/events/ready.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::emitters::dest, paths::emitters::shutdown)?,
            include_str!("./static/implementation/events/shutdown.ts").to_owned(),
            true,
        )?;
        helpers::fs::write(
            self.get_dest_file(base, paths::index::dest, paths::index::module)?,
            include_str!("./static/index.ts").to_owned(),
            true,
        )?;
        let context_dest =
            self.get_dest_file(base, paths::context::dest, paths::context::module)?;
        if !context_dest.exists() {
            helpers::fs::write(
                self.get_dest_file(base, paths::context::dest, paths::context::module)?,
                include_str!("./static/context.ts").to_owned(),
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
