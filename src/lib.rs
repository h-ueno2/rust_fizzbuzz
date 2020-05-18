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
    pub fn make_answer(&self, num: u32) -> String {
        match num {
            num if num % self.fizz_buzz_num == 0 => String::from("FizzBuzz"),
            num if num % self.fizz_num == 0 => String::from("Fizz"),
            num if num % self.buzz_num == 0 => String::from("Buzz"),
            _ => num.to_string(),
        }
    }

    /// 1からmaxまでの値を対象にFizzBuzzを実行します。
    pub fn run(&self) {
        for i in 1..=self.max {
            println!("{}", self.make_answer(i));
        }
    }
}

#[cfg(test)]
mod make_answer {
    use super::*;

    #[test]
    fn return_fizz() {
        let manager = Manager::new(3, 5, 10);
        let expected = String::from("Fizz");
        assert_eq!(manager.make_answer(3), expected);
        assert_eq!(manager.make_answer(6), expected);
    }

    #[test]
    fn return_buzz() {
        let manager = Manager::new(3, 5, 10);
        let expected = String::from("Buzz");
        assert_eq!(manager.make_answer(5), expected);
        assert_eq!(manager.make_answer(10), expected);
    }

    #[test]
    fn return_fizz_buzz() {
        let manager = Manager::new(3, 5, 10);
        let expected = String::from("FizzBuzz");
        assert_eq!(manager.make_answer(15), expected);
        assert_eq!(manager.make_answer(30), expected);
    }

    #[test]
    fn return_number() {
        let manager = Manager::new(3, 5, 10);
        assert_eq!(manager.make_answer(2), String::from("2"));
        assert_eq!(manager.make_answer(4), String::from("4"));
    }
}
