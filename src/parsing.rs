#[derive(Debug)]
pub struct Data {
    pub nbr_philo: u32,
    pub time_to_die: u32,
    pub time_to_eat: u32,
    pub time_to_sleep: u32,
    pub nbr_meal: u32,
    meal_enable: bool,
}

impl Data {
    fn new() -> Self {
        Data { nbr_philo: 0, time_to_die: 0, time_to_eat: 0, time_to_sleep: 0, nbr_meal: 0, meal_enable: false }
    }

    pub fn build(args: Vec<String>) -> Result<Data, String> {
        if args.len() < 5 && args.len() > 6 {
            eprintln!("Error: Wrong number of argument!\nusage: cargo r -- nbr_philo time_to_die time_to_eat time_to_sleep [nbr_meal]");
            std::process::exit(1);
        }

        let mut data: Data = Data::new();

        if let 6 = args.len() {
            data.meal_enable = true;
        }

        for (i, arg) in args.into_iter().skip(1).enumerate() {
            match arg.parse() {
                Ok(value) => {
                    match i {
                        0 => data.nbr_philo = value,
                        1 => data.time_to_die = value,
                        2 => data.time_to_eat = value,
                        3 => data.time_to_sleep = value,
                        4 => data.nbr_meal = value,
                        _ => continue
                    }
                }
                Err(err) => {
                    // eprintln!("Error: {err}\nArgument {arg} is not a valid one!");
                    return Err(format!("Error: {err}\nArgument {arg} is not a valid one!"));
                }
            }
        }
        Ok(data)
    }

    pub fn check_value(&self) -> Result<(), String> {
        if self.nbr_philo == 0 {
            return Err("Error: the number of philosophers must be positive".to_string());
        }
        if self.time_to_die == 0 {
            return Err("Error: time_to_die must be different from 0".to_string());
        }
        if self.time_to_eat == 0 {
            return Err("Error: time_to_eat must be different from 0".to_string());
        }
        if self.time_to_sleep == 0 {
            return Err("Error: time_to_sleep must be different from 0".to_string());
        }
        Ok(())
    }
}
