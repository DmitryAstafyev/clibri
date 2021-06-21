use super::{
    helpers::{
        render as tools,
    },
    workflow::{
        request::{
            Request
        }
    }
};

mod templates {
    pub const MODULE: &str = 
r#"group [[name]]
    Consumer -> Producer: [[request]]
    Producer -->x Consumer: <font color=red>[[error]][[conclusions]]
end"#;
}

pub struct RenderRequest {
}

impl Default for RenderRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderRequest {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        request: &Request
    ) -> Result<String, String> {
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[name]]", &request.as_struct_name()?);
        output = output.replace("[[request]]", &request.get_request()?);
        output = output.replace("[[error]]", &request.get_err()?);
        output = output.replace("[[conclusions]]", &tools::inject_tabs(1, self.get_conclusions(request)?));
        Ok(output)
    }

    fn get_conclusions(&self, request: &Request) -> Result<String, String> {
        let mut output: String = String::new();
        if request.actions.len() > 1 {
            for action in &request.actions {
                let name = if let Some(name) = action.conclusion.as_ref() {
                    name
                } else {
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
                let reference = if let Some(reference) = action.response.as_ref() {
                    reference
                } else {
                    return Err(String::from("Action doesn't have bound response reference"));
                };
                let mut broadcasts: String = String::new();
                for broadcast in &action.broadcast {
                    broadcasts = format!("{}\nProducer {} Consumers: {}",
                        broadcasts,
                        if broadcast.optional { "-->" } else { "->" },
                        broadcast.reference,
                    );
                }
                output = format!(
r#"{}
== {} ==
    Producer -> Consumer: {}{}"#,
                    output,
                    name,
                    reference,
                    tools::inject_tabs(1, broadcasts),
                );
            }
        } else {
            output = format!("\nProducer -> Consumer: {}", request.get_response()?);
        }
        Ok(output)
    }

}

