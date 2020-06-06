extern crate fizz_buzz;

use std::env;
use fizz_buzz::Manager;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max = parse_config(&args);
    let result = Manager::new(3, 5, max).run();
    println!("{}", result);
}

fn parse_config(args: &[String]) -> u32 {
    let default_max = 100;
    if args.len() <= 1 {
        return default_max
    }
    let max: u32 = args[1].parse().unwrap_or_else(|_| {
        default_max
    });
    max
}
