pub struct Config {
    max: u32,
    fizz_num: u32,
    buzz_num: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let default_max = 100;
        let default_fizz = 3;
        let default_buzz = 5;

        match args.len() {
            i if i == 2 => Config {
                max: args[1].parse().unwrap_or_else(|_| default_max),
                fizz_num: default_fizz,
                buzz_num: default_buzz,
            },
            i if i == 3 => Config {
                max: args[1].parse().unwrap_or_else(|_| default_max),
                fizz_num: args[2].parse().unwrap_or_else(|_| default_fizz),
                buzz_num: default_buzz,
            },
            i if i == 4 => Config {
                max: args[1].parse().unwrap_or_else(|_| default_max),
                fizz_num: args[2].parse().unwrap_or_else(|_| default_fizz),
                buzz_num: args[3].parse().unwrap_or_else(|_| default_buzz),
            },
            _ => Config {
                max: default_max,
                fizz_num: default_fizz,
                buzz_num: default_buzz,
            },
        }
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }

    pub fn get_fizz(&self) -> u32 {
        self.fizz_num
    }

    pub fn get_buzz(&self) -> u32 {
        self.buzz_num
    }
}
