#[path = "./ctrl.args.rs"]
pub mod ctrlargs;

fn main() {
    let path = env::current_dir()?;

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
