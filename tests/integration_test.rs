extern crate fizz_buzz;

use fizz_buzz::Manager;
#[test]
fn fizzbuzz_manager_01() {
    let manager = Manager::new(3, 5, 15);
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
    let manager = Manager::new(2, 3, 10);
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
