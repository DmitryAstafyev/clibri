use std::fmt;

pub struct Stat {
    connecting: usize,      // Attempt to connect
    connected: usize,       // Socket accepted and connected
    disconnected: usize,    // Socket disconnected
    errors: usize,
    recieved_bytes: usize,
    recieved_packages: usize,
    sent_bytes: usize,
    sent_packages: usize,
    alive: usize,
}

impl Stat {

    pub fn new() -> Self {
        Self {
            connecting: 0,
            connected: 0,
            disconnected: 0,
            errors: 0,
            recieved_bytes: 0,
            recieved_packages: 0,
            sent_bytes: 0,
            sent_packages: 0,
            alive: 0, 
        }
    }

    pub fn connecting(&mut self) -> () {
        self.connecting += 1;
    }

    pub fn connected(&mut self) -> () {
        self.connected += 1;
    }

    pub fn disconnected(&mut self) -> () {
        self.disconnected += 1;
    }

    pub fn errors(&mut self) -> () {
        self.errors += 1;
    }

    pub fn recieved_bytes(&mut self, bytes: usize) -> () {
        self.recieved_bytes += bytes;
        self.recieved_packages += 1;
    }

    pub fn sent_bytes(&mut self, bytes: usize) -> () {
        self.sent_bytes += bytes;
        self.sent_packages += 1;
    }

    pub fn alive(&mut self, alive: usize) -> () {
        self.alive = alive;
    }

    pub fn print(&self) -> () {
        println!("{}", self);
    }


}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\
- connecting:       {} attempts
- connected:        {} clients
- disconnected:     {} clients
- errors:           {} has been gotten
- recieved:         {} bytes
- recieved:         {} packages
- sent:             {} bytes
- sent:             {} packages
- alive:            {} clients
",
            self.connecting,
            self.connected,
            self.disconnected,
            self.errors,
            self.recieved_bytes,
            self.recieved_packages,
            self.sent_bytes,
            self.sent_packages,
            self.alive,
        )
    }
}