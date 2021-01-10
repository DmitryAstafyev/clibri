use uuid::Uuid;

pub trait ConnectionContext {
    /** Sends buffer to active connection */
    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String>;

    /** Sends buffer to defined connection */
    fn send_to(&mut self, uuid: Uuid, buffer: Vec<u8>) -> Result<(), String>;
}
