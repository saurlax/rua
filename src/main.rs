use std::env;

mod parse;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
    println!("{}", args[1]);
}
