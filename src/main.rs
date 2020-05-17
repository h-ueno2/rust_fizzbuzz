extern crate fizz_buzz;

use fizz_buzz::Manager;

fn main() {
    Manager::new(3, 5, 100).run();
}
