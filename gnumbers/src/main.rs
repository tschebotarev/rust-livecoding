use std::env;

mod generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => {},
        2 => {
            if let Ok(n) = args[1].parse() {
                generator::generate(n)
            }
        }
        _ => {}
    }
}
