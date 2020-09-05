pub trait Protocol<T> {
    
    fn get_msg(&self, id: u32, payload: &str) -> Result<T, String>;
    
    fn get_payload_limit(&self, id: u32) -> Result<u32, String>;

}