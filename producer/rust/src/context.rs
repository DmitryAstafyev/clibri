pub trait Connection {
    
    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String>;

}

pub trait Context<Identification> {

    fn connection(&mut self) -> Option<&'static mut dyn Connection>;

    fn connections(&mut self, ident: Identification) -> Option<Vec<&'static mut dyn Connection>>;

}
