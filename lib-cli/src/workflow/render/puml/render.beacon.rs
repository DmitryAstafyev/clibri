use super::{helpers::render as tools, workflow::beacon::Broadcast};

mod templates {
    pub const MODULE: &str = r#"group Beacons
    Consumer -> Producer: Connected Event
    Consumer -> Producer: Disconnected Event
    Controller -> Producer: Direct call[[broadcasts]]
end"#;
}

pub struct RenderBeacons {}

impl Default for RenderBeacons {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBeacons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, beacons: &Vec<Broadcast>) -> Result<String, String> {
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace(
            "[[broadcasts]]",
            &tools::inject_tabs(1, self.get_beacons(beacons)?),
        );
        Ok(output)
    }

    fn get_beacons(&self, beacons: &Vec<Broadcast>) -> Result<String, String> {
        let mut output: String = String::new();
        for broadcast in beacons {
            output = format!("{}\nConsumer --> Producer: {}", output, broadcast.reference,);
        }
        Ok(output)
    }
}
