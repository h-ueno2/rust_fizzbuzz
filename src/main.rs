fn main() {
    for i in 1..101 {
        let answer = match i {
            i if i % 15 == 0 => String::from("FizzBuzz"),
            i if i % 3 == 0 => String::from("Fizz"),
            i if i % 5 == 0 => String::from("Buzz"),
            _ => i.to_string(),
        };
        println!("{}",answer);
    }
}
