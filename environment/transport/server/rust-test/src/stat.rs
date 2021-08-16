pub struct Stat {
    pub created: u32,
    pub destroyed: u32,
    pub connected: u32,
    pub disconnected: u32,
    pub client_connected: u32,
    pub client_done: u32,
    pub failed: u32,
    pub sent: u32,
    pub write: u32,
    pub read: u32,
    pub recieved: u32,
    pub wakeup: u32,
    pub created_in: u128,
    pub sent_in: u128,
    pub done_in: u128,
}

impl Default for Stat {
    fn default() -> Self {
        Self::new()
    }
}

impl Stat {
    pub fn new() -> Self {
        Self {
            created: 0,
            destroyed: 0,
            connected: 0,
            disconnected: 0,
            client_connected: 0,
            client_done: 0,
            failed: 0,
            sent: 0,
            read: 0,
            write: 0,
            recieved: 0,
            wakeup: 0,
            created_in: 0,
            sent_in: 0,
            done_in: 0,
        }
    }
    pub fn print(&self) {
        println!("==========================================================================");
        println!("Clients created:                  {}", self.created);
        println!("Clients destroyed:                {}", self.destroyed);
        println!("Clients connected:                {}", self.connected);
        println!("Clients disconnected:             {}", self.disconnected);
        println!(
            "Clients connected (client):       {}",
            self.client_connected
        );
        println!("Clients done (client):            {}", self.client_done);
        println!("Clients failed:                   {}", self.failed);
        println!("Clients wakeup:                   {}", self.wakeup);
        println!("Packages write:                   {}", self.write);
        println!("Packages sent:                    {}", self.sent);
        println!("Packages read:                    {}", self.read);
        println!("Packages recieved:                {}", self.recieved);
        println!("Created in:                       {}ms", self.created_in);
        println!("Sent in:                          {}ms", self.sent_in);
        println!("Done in:                          {}ms", self.done_in);
        println!("==========================================================================");
    }
}
