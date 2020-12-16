use std::env;

#[path = "./writer.rs"]
pub mod writer;

#[path = "./reader.rs"]
pub mod reader;

use writer::{ write };
use reader::{ read };

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args[0] == "write" {
        match write() {
            Ok(_) => {
    
            },
            Err(e) => panic!(e)
        }
    } else if args[0] == "read" {
        match read() {
            Ok(_) => {
    
            },
            Err(e) => panic!(e)
        }
    }

}
