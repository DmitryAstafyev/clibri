
pub trait Protocol<T> {
    
    fn get_msg(&self, id: u32, buffer: &[u8]) -> Result<T, String>;
    
}