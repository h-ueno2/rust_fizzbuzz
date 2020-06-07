pub struct Config {
    max: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let default_max = 100;
        if args.len() <= 1 {
            return Config { max: default_max };
        }
        let max: u32 = args[1].parse().unwrap_or_else(|_| default_max);
        Config { max: max }
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }
}
