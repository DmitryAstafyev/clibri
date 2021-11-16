use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { emit } from "../../beacons/[[beacon_mod]]";

export function handler(
    beacon: Protocol.[[beacon]],
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return emit(beacon, consumer, filter, context, producer).then(() => {
        const confirmation =
            new Protocol.InternalServiceGroup.BeaconConfirmation({
                error: undefined,
            });
        return producer.send(
            consumer.uuid(),
            confirmation.pack(sequence, consumer.uuid())
        );
    });
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

    pub fn render(&self, base: &Path, beacon: &Broadcast) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, beacon)?;
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[beacon]]", &beacon.reference);
        output = output.replace("[[beacon_mod]]", &beacon.reference.to_lowercase());
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, beacon: &Broadcast) -> Result<PathBuf, String> {
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
        Ok(dest.join(format!("{}.ts", beacon.reference.to_lowercase())))
    }
}
