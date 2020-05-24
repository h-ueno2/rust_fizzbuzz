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

/// `Manager`は`FizzBuzz`ゲームを管理します。
pub struct Manager {
    fizz_num: u32,
    buzz_num: u32,
    fizz_buzz_num: u32,
    max: u32,
}

impl Manager {
    /// 新しい`Manager`を作成します。引数に応じてルールを設定します。
    ///
    /// # Argments
    ///
    /// * `fizz_num` - 値を`Fizz`とするための基準値
    /// * `buzz_num` - 値を`Buzz`とするための基準値
    /// * `max` - FizzBuzzゲームを繰り返し行う最大値
    pub fn new(fizz_num: u32, buzz_num: u32, max: u32) -> Manager {
        let fizz_buzz_num = fizz_num * buzz_num;
        Manager {
            fizz_num,
            buzz_num,
            fizz_buzz_num,
            max,
        }
    }

    /// 与えられた数値から回答を作成して返却します。
    ///
    /// # Arguments
    ///
    /// * `num` - 対象の数値
    ///
    /// # Return value
    ///
    /// 返却値は`num`によって以下の値をとります。
    ///
    /// * `fizz_num`で割り切れる数字であった場合は`Fizz`を返却します。
    /// * `buzz_num`で割り切れる数字であった場合は`Buzz`を返却します。
    /// * `fizz_num`及び`buzz_num`の両方で割り切れる値であった場合は`FizzBuzz`を返却します。
    /// * 上記以外の値で割り切れる値であった場合は`num`を返却します。
    pub fn make_answer(&self, num: u32) -> Answer {
        match num {
            num if num % self.fizz_buzz_num == 0 => Answer::FizzBuzz,
            num if num % self.fizz_num == 0 => Answer::Fizz,
            num if num % self.buzz_num == 0 => Answer::Buzz,
            _ => Answer::Number(num),
        }
    }

    /// 1からmaxまでの値を対象にFizzBuzzを実行します。
    /// 
    /// # Return value
    /// 
    /// `max`までのFizzBuzzの実行結果を返却します。
    /// * 値ごとに改行されます。
    /// 
    pub fn run(&self) -> String {
        let mut result = String::from("");
        for i in 1..=self.max {
            let answer = self.make_answer(i);
            result = match i {
                1 => format!("{}", answer),
                _ => format!("{}\n{}", result, answer),
            }
        }
        result
    }
}

#[cfg(test)]
mod make_answer {
    use super::*;

    #[test]
    fn return_fizz() {
        let manager = Manager::new(3, 5, 10);
        let expected = Answer::Fizz;
        assert_eq!(manager.make_answer(3), expected);
        assert_eq!(manager.make_answer(6), expected);
    }

    #[test]
    fn return_buzz() {
        let manager = Manager::new(3, 5, 10);
        let expected = Answer::Buzz;
        assert_eq!(manager.make_answer(5), expected);
        assert_eq!(manager.make_answer(10), expected);
    }

    #[test]
    fn return_fizz_buzz() {
        let manager = Manager::new(3, 5, 10);
        let expected = Answer::FizzBuzz;
        assert_eq!(manager.make_answer(15), expected);
        assert_eq!(manager.make_answer(30), expected);
    }

    #[test]
    fn return_number() {
        let manager = Manager::new(3, 5, 10);
        assert_eq!(manager.make_answer(2), Answer::Number(2));
        assert_eq!(manager.make_answer(4), Answer::Number(4));
    }
}

#[cfg(test)]
mod run {
    use super::*;

    #[test]
    fn result() {
        let manager = Manager::new(3,5,10);
        let expected = "1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz";
        assert_eq!(manager.run(), expected);
    }
}
