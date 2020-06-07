extern crate fizz_buzz;

use fizz_buzz::Config;
use fizz_buzz::Manager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max = Config::new(&args).get_max();
    let result = Manager::new(3, 5, max).run();
    println!("{}", result);
}
