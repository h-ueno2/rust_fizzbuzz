use std::fmt;

#[derive(Debug,PartialEq)]
pub enum Answer {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Answer::Fizz => write!(f, "Fizz"),
            Answer::Buzz => write!(f, "Buzz"),
            Answer::FizzBuzz => write!(f, "FizzBuzz"),
            Answer::Number(n) => write!(f, "{}", n),
        }
    }
}

