extern crate fizz_buzz;

use fizz_buzz::Config;
use fizz_buzz::Manager;
#[test]
fn fizzbuzz_manager_01() {
    let config = Config::new(&[
        String::new(),
        String::from("15"),
        String::from("3"),
        String::from("5"),
    ]);
    let manager = Manager::new(config);
    let expected = "1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz";
    assert_eq!(manager.run(), expected);
}

#[test]
fn fizzbuzz_manager_02() {
    let config = Config::new(&[
        String::new(),
        String::from("10"),
        String::from("2"),
        String::from("3"),
    ]);
    let manager = Manager::new(config);
    let expected = "1
Fizz
Buzz
Fizz
5
FizzBuzz
7
Fizz
Buzz
Fizz";
    assert_eq!(manager.run(), expected);
}
