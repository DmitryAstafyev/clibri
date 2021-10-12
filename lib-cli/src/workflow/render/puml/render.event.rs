use super::{helpers::render as tools, workflow::event::Event};

mod templates {
    pub const MODULE: &str = r#"group [[name]]
    Producer -> Producer: [[reference]][[broadcasts]]
end"#;
}

pub struct RenderEvent {}

impl Default for RenderEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEvent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, event: &Event) -> Result<String, String> {
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[name]]", &event.get_reference()?);
        output = output.replace("[[reference]]", &event.get_reference()?);
        output = output.replace(
            "[[broadcasts]]",
            &tools::inject_tabs(1, self.get_broadcasts(event)?),
        );
        Ok(output)
    }

    fn get_broadcasts(&self, event: &Event) -> Result<String, String> {
        let mut output: String = String::new();
        for broadcast in &event.broadcasts {
            output = format!("{}\nProducer --> Consumer: {}", output, broadcast.reference,);
        }
        Ok(output)
    }
}
