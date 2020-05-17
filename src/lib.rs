pub struct Manager {
    fizz_num: u32,
    buzz_num: u32,
    fizz_buzz_num: u32,
    max: u32,
}

impl Manager {
    pub fn new(fizz:u32, buzz:u32, max: u32) -> Manager {
        let fizz_num = fizz;
        let buzz_num = buzz;
        let fizz_buzz_num = fizz_num * buzz_num;
        let max = max + 1;
        Manager {
            fizz_num,
            buzz_num,
            fizz_buzz_num,
            max,
        }
    }

    pub fn make_answer(&self, num: u32) -> String {
        match num {
            num if num % self.fizz_buzz_num == 0 => String::from("FizzBuzz"),
            num if num % self.fizz_num == 0 => String::from("Fizz"),
            num if num % self.buzz_num == 0 => String::from("Buzz"),
            _ => num.to_string(),
        }
    }

    pub fn run(&self) {
        for i in 1..self.max {
            println!("{}", self.make_answer(i));
        }
    }
}
