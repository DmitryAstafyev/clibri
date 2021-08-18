use std::fmt;

pub struct Stat {
    connection_type: String,
    connecting: usize,   // Attempt to connect
    connected: usize,    // Socket accepted and connected
    disconnected: usize, // Socket disconnected
    listeners_created: u16,
    listeners_destroyed: u16,
    errors: usize,
    recieved_bytes: usize,
    recieved_packages: usize,
    sent_bytes: usize,
    sent_packages: usize,
    alive: usize,
}

impl Default for Stat {
    fn default() -> Self {
        Self::new()
    }
}

impl Stat {
    pub fn new() -> Self {
        Self {
            connection_type: String::new(),
            connecting: 0,
            connected: 0,
            disconnected: 0,
            listeners_created: 0,
            listeners_destroyed: 0,
            errors: 0,
            recieved_bytes: 0,
            recieved_packages: 0,
            sent_bytes: 0,
            sent_packages: 0,
            alive: 0,
        }
    }

    pub fn connecting(&mut self) {
        self.connecting += 1;
    }

    pub fn connected(&mut self) {
        self.connected += 1;
    }

    pub fn disconnected(&mut self) {
        self.disconnected += 1;
    }

    pub fn errors(&mut self) {
        self.errors += 1;
    }

    pub fn recieved_bytes(&mut self, bytes: usize) {
        self.recieved_bytes += bytes;
        self.recieved_packages += 1;
    }

    pub fn sent_bytes(&mut self, bytes: usize) {
        self.sent_bytes += bytes;
        self.sent_packages += 1;
    }

    pub fn alive(&mut self, alive: usize) {
        self.alive = alive;
    }

    pub fn listener_created(&mut self) {
        self.listeners_created += 1;
    }

    pub fn listener_destroyed(&mut self) {
        self.listeners_destroyed += 1;
    }

    pub fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
- server:           {}
- listeners created {}
- listeners destroy {}
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
            self.connection_type,
            self.listeners_created,
            self.listeners_destroyed,
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
