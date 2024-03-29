use super::helpers;

use std::path::{Path, PathBuf};

mod templates {
    pub const MODULE: &str = r#"import { Logger, DefaultLogger } from 'clibri';

export interface IOptions {
    logger?: Logger;
    autoconnect?: boolean;
    reconnect?: number;
    global?: boolean;
}

export class Options {

    public autoconnect: boolean = true;
    public global: boolean = true;
    public reconnect: number = 2000;
    public logger: Logger;

    constructor(alias: string, options: IOptions = {}) {
        if (options.logger !== undefined) {
            this.logger = options.logger;
        } else {
            this.logger = new DefaultLogger(alias);
        }
        options.autoconnect !== undefined && (this.autoconnect = options.autoconnect);
        options.reconnect !== undefined && (this.reconnect = options.reconnect);
        options.global !== undefined && (this.global = options.global);
    }

}"#;
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
        let dest: PathBuf = self.get_dest_file(base);
        helpers::fs::write(dest, templates::MODULE.to_owned(), true)
    }

    fn get_dest_file(&self, base: &Path) -> PathBuf {
        base.join("options.ts")
    }
}
