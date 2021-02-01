pub trait Logger {

    fn warn(&self, str: &str) -> () {
        println!("WARN: {}", str);
    }

    fn err(&self, str: &str) -> () {
        println!("ERROR: {}", str);
    }

    fn debug(&self, str: &str) -> () {
        println!("DEBUG: {}", str);
    }

    fn verb(&self, str: &str) -> () {
        println!("VERB: {}", str);
    }

}