use super::{
    Event,
    Request,
    Config,
    Protocol,
};

#[derive(Debug, Clone)]
pub struct Store {
    pub events: Vec<Event>,
    pub requests: Vec<Request>,
    pub config: Option<Config>,
}

impl Store {

    pub fn new() -> Self {
        Self {
            events: vec![],
            requests: vec![],
            config: None,
        }
    }

    pub fn set_config(&mut self, config: Config) -> Result<(), String> {
        if self.config.is_some() {
            Err(String::from("Configuration can be defined only once"))
        } else {
            self.config = Some(config);
            Ok(())
        }
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), String> {
        let reference = if let Some(reference) = event.reference.as_ref() {
            reference
        } else {
            return Err(String::from("Fail to add event without reference to object/struct"));
        };
        for stored in &self.events {
            if let Some(stored) = stored.reference.as_ref() {
                if stored == reference {
                    return Err(format!("Event with reference {} has been added already.", reference));
                }      
            }
        }
        self.events.push(event);
        Ok(())
    }

    pub fn add_request(&mut self, request: Request) -> Result<(), String> {
        let reference = if let Some(reference) = request.request.as_ref() {
            reference
        } else {
            return Err(String::from("Fail to add request without reference to request object/struct"));
        };
        for stored in &self.requests {
            if let Some(stored) = stored.request.as_ref() {
                if stored == reference {
                    return Err(format!("Request with reference {} has been added already.", reference));
                }   
            }
        }
        self.requests.push(request);
        Ok(())
    }

    pub fn get_config(&self) -> Result<&Config, String> {
        if let Some(config) = self.config.as_ref() {
            Ok(config)
        } else {
            Err(String::from("Config isn't defined for workflow"))
        }
    }
}