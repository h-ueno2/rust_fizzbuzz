/// `Rule`は`FizzBuzz`ゲームのルールとして以下を保持します。
/// * `fizz_num` - `Fizz`に対応する値
/// * `buzz_bum` - `Buzz`に対応する値
pub struct Rule {
    fizz_num: u32,
    buzz_num: u32,
}

impl Rule {
    /// 新しい`Rule`を作成します。
    /// 
    /// # Argments
    ///
    /// * `fizz_num` - 値を`Fizz`とするための基準値
    /// * `buzz_num` - 値を`Buzz`とするための基準値
    pub fn new(fizz_num: u32, buzz_num: u32) -> Self {
        Self {
            fizz_num,
            buzz_num,
        }
    }

    /// FizzBuzzに対応する値を返却します。
    /// 
    /// # Return value
    ///
    /// FizzBuzzに対応する値。
    pub fn get_fizzbuzz_num(&self) -> u32 {
        self.fizz_num * self.buzz_num
    }

    pub fn get_fizz_num(&self) -> u32 {
        self.fizz_num
    }

    pub fn get_buzz_num(&self) -> u32 {
        self.buzz_num
    }
} 

#[cfg(test)]
mod fizz_buzz_num {
    use super::*;

    #[test]
    fn return_fizzbuzz() {
        let rule = Rule {
            fizz_num: 3,
            buzz_num: 5,
        };
        assert_eq!(rule.get_fizzbuzz_num(), 15);

        let rule = Rule {
            fizz_num: 4,
            buzz_num: 5,
        };
        assert_eq!(rule.get_fizzbuzz_num(), 20);
    }
}
