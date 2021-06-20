use super::{
    helpers::{
        render as tools,
    },
    workflow::{
        broadcast::{
            Broadcast
        }
    }
};

mod templates {
    pub const MODULE: &str = 
r#"group Broadcasts
    Consumer -> Producer: Connected Event
    Consumer -> Producer: Disconnected Event
    Controller -> Producer: Direct call[[broadcasts]]
end"#;
}

pub struct RenderBroadcasts {
}

impl Default for RenderBroadcasts {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBroadcasts {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        broadcasts: &Vec<Broadcast>,
    ) -> Result<String, String> {
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[broadcasts]]", &tools::inject_tabs(1, self.get_broadcasts(broadcasts)?));
        Ok(output)
    }

    fn get_broadcasts(&self, broadcasts: &Vec<Broadcast>) -> Result<String, String> {
        let mut output: String = String::new();
        for broadcast in broadcasts {
            output = format!("{}\nProducer --> Consumer: {}",
                output,
                broadcast.reference,
            );
        }
        Ok(output)
    }

}

