#[allow(unused)]
#[allow(dead_code)]

mod parsing;

use std::{
    env,
    time::{Duration, Instant},
    thread,
};

use std::sync::{
    Arc, 
    Mutex
};

use parsing::Data;

#[derive(Debug)]
struct Philosopher {
    id: usize,
    left: usize,
    right: usize,
    start_time: Instant,
    time_to_die: u128,
    table: Arc<Data>,
}

enum Speak {
    Dead,
    Think,
    Eat,
    Sleep,
    Done,
    Fork,
}

impl Philosopher {
    fn new(id: usize, left: usize, right: usize, time_to_die: u128, table: Arc<Data>) -> Philosopher {
        Philosopher { 
            id: id, 
            left: left, 
            right: right, 
            start_time: Instant::now(), 
            time_to_die: time_to_die, 
            table: table }
    }

    fn run(&mut self) {
        'dead: loop {
            if !self.is_alive() {
                break 'dead;
            }
            self.speak(Speak::Think);


            let left = self.table.forks[self.left].lock().unwrap();
            self.speak(Speak::Fork);
            let right = self.table.forks[self.right].lock().unwrap();
            self.speak(Speak::Fork);


            if !self.is_alive() {
                break 'dead;
            } else {
                self.time_to_die += self.start_time.elapsed().as_millis();
            }
            self.speak(Speak::Eat);

            thread::sleep(Duration::from_millis(self.table.time_to_eat));
            if !self.is_alive() {
                break 'dead;
            }
            drop(left);
            drop(right);

            if !self.is_alive() {
                break 'dead;
            }
            self.speak(Speak::Sleep);
            thread::sleep(Duration::from_millis(self.table.time_to_sleep));
        }
    }

    fn speak(&self, sentence: Speak) {
        let time = self.start_time.elapsed().as_millis();
        match sentence {
            Speak::Done => println!("[{}ms] {} is done eating!", time, self.id),
            Speak::Think => println!("[{}ms] {} is thinking!", time, self.id),
            Speak::Eat => println!("[{}ms] {} is eating!", time, self.id),
            Speak::Sleep => println!("[{}ms] {} is sleeping!", time, self.id),
            Speak::Fork => println!("[{}ms] {} has taken a fork!", time, self.id),
            _ => {}
        }
    }

    fn is_alive(&self) -> bool {
        let time = self.start_time.elapsed().as_millis();
        if time > self.time_to_die {
            println!("[{}ms] {} died", self.time_to_die, self.id);
            return false;
        }
        true
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let data: Data = match Data::build(args) {
        Ok(info) => info,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    let nbr_philo = data.nbr_philo;
    let time_to_die = data.time_to_die;
    let table: Arc<Data> = Arc::new(data);

    let philosophers: Vec<Philosopher> = (0..nbr_philo as usize).into_iter().map(|i| {
        let left = i;
        let right = (i + 1) % nbr_philo as usize;
        Philosopher::new(i, left, right, time_to_die, Arc::clone(&table))
    }).collect();

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|mut philo| {
            thread::spawn(move || {
                philo.run();
            })
        })
    .collect();

    for handle in handles {
        handle.join().unwrap();
    }

}
