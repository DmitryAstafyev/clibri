use uuid::Uuid;

pub enum Control {
    Shutdown,
    Disconnect(Uuid)
}
