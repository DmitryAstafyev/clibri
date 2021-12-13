use super::{Beacons, Broadcast, Config, Event, Request};

#[derive(Debug, Clone)]
pub struct Store {
    pub events: Vec<Event>,
    pub requests: Vec<Request>,
    pub beacons: Vec<Broadcast>,
    pub config: Option<Config>,
    hash: String,
}

impl Store {
    pub fn new(hash: String) -> Self {
        Self {
            events: vec![],
            requests: vec![],
            beacons: vec![],
            config: None,
            hash,
        }
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
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
            return Err(String::from(
                "Fail to add event without reference to object/struct",
            ));
        };
        for stored in &self.events {
            if let Some(stored) = stored.reference.as_ref() {
                if stored == reference {
                    return Err(format!(
                        "Event with reference {} has been added already.",
                        reference
                    ));
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
            return Err(String::from(
                "Fail to add request without reference to request object/struct",
            ));
        };
        for stored in &self.requests {
            if let Some(stored) = stored.request.as_ref() {
                if stored == reference {
                    return Err(format!(
                        "Request with reference {} has been added already.",
                        reference
                    ));
                }
            }
        }
        self.requests.push(request);
        Ok(())
    }

    pub fn add_beacon(&mut self, mut beacons: Beacons) -> Result<(), String> {
        while let Some(beacon) = beacons.next_beacon() {
            for stored in &self.beacons {
                if stored.reference == beacon.reference {
                    return Err(format!(
                        "Broadcast with reference {} has been added already.",
                        beacon.reference
                    ));
                }
            }
            self.beacons.push(beacon.clone());
        }
        Ok(())
    }

    pub fn get_config(&self) -> Result<&Config, String> {
        if let Some(config) = self.config.as_ref() {
            Ok(config)
        } else {
            Err(String::from("Config isn't defined for workflow"))
        }
    }

    fn is_request_exist(&self, req: String) -> Result<bool, String> {
        for request in &self.requests {
            if request.get_request()? == req {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn validate(&self) -> Result<(), String> {
        // Beacons and requests should not be in conflict
        for beacon in &self.beacons {
            if self.is_request_exist(beacon.reference.clone())? {
                return Err(format!("Beacon \"{}\" also is used as request. Beacons should use unique structs/enums", beacon.reference));
            }
        }
        Ok(())
    }
}
