#[path = "./writer.rs"]
pub mod writer;

use writer::{ write };

fn main() {
    match write() {
        Ok(_) => {

        },
        Err(e) => panic!(e)
    }
}
