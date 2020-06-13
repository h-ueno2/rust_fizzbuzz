extern crate fizz_buzz;

use fizz_buzz::Config;
use fizz_buzz::Manager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    Manager::new(config).run_callback(|answer| {
        println!("{}", answer);
    });
}
