extern crate fizz_buzz;

use std::env;
use fizz_buzz::Manager;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max: u32 = args[1].parse().unwrap();
    let result = Manager::new(3, 5, max).run();
    println!("{}", result);
}
