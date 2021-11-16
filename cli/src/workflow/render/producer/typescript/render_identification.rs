use super::{
    helpers, helpers::render as tools, workflow::config::Config, workflow::store::Store, Protocol,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import { ProducerIdentificationStrategy, Logger } from "fiber";
import * as Protocol from "../protocol";

export class Identification {
    private readonly _uuid: string;
    private readonly _strategy: ProducerIdentificationStrategy;
    private _discredited: boolean = false;
    private _key: Protocol.[[self_key]] | undefined;
    private _assigned: Protocol.[[assign_key]] | undefined;
    private _logger: Logger;

    constructor(
        uuid: string,
        strategy: ProducerIdentificationStrategy,
        logger: Logger
    ) {
        this._uuid = uuid;
        this._strategy = strategy;
        this._logger = logger.clone(`[${uuid}][Identification]`);
    }

    public uuid(): string {
        return this._uuid;
    }

    public key(
        key: Protocol.[[self_key]],
        overwrite: boolean
    ): string {
        if (this._key === undefined || overwrite) {
            this._key = key;
        } else {[[self_key_overwrite]]
        }
        return this._uuid;
    }

    public assign(
        key: Protocol.[[assign_key]],
        overwrite: boolean
    ) {
        if (this._assigned === undefined || overwrite) {
            this._assigned = key;
        } else {[[assign_key_overwrite]]
        }
    }

    public assigned(): boolean {
        if (this.assign === undefined) {
            switch (this._strategy) {
                case ProducerIdentificationStrategy.Ignore:
                    return true;
                case ProducerIdentificationStrategy.Log:
                    this._logger.warn(`Consumer ${this._uuid} isn't assigned`);
                    return true;
                default:
                    return false;
            }
        } else {
            return true;
        }
    }

    public hasKey(): boolean {
        return this._key !== undefined;
    }

    public discredited() {
        this._discredited = true;
    }

    public isDiscredited(): boolean {
        return this._discredited;
    }
}"#;
    pub const ASSIGNED_CHECK: &str = r#"if (key.[[field]] !== undefined) {
        this._assigned.[[field]] = key.[[field]];
}"#;
    pub const KEY_CHECK: &str = r#"if (key.[[field]] !== undefined) {
    this._key.[[field]] = key.[[field]];
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

    pub fn render(&self, base: &Path, store: &Store, protocol: &Protocol) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = templates::MODULE.to_owned();
        let config = store.get_config()?;
        output = output.replace("[[assign_key]]", &store.get_config()?.get_assigned()?);
        output = output.replace("[[self_key]]", &store.get_config()?.get_self()?);
        output = output.replace(
            "[[self_key_overwrite]]",
            &tools::inject_tabs(3, self.self_key_overwrite(&config, protocol)?),
        );
        output = output.replace(
            "[[assign_key_overwrite]]",
            &tools::inject_tabs(3, self.assign_key_overwrite(&config, protocol)?),
        );
        helpers::fs::write(dest, output, true)
    }

    fn self_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_self()?) {
            let mut output: String = String::new();
            for field in strct.fields.iter() {
                output = format!(
                    "{}\n{}",
                    output,
                    templates::KEY_CHECK.replace("[[field]]", &field.name)
                );
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
    }

    fn assign_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_assigned()?) {
            let mut output: String = String::new();
            for field in strct.fields.iter() {
                output = format!(
                    "{}\n{}",
                    output,
                    templates::ASSIGNED_CHECK.replace("[[field]]", &field.name)
                );
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("consumer");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("identification.ts"))
    }
}
