extern crate rand;

use std::{env, fmt, str::FromStr};
use rand::Rng;

#[derive(Debug)]
struct Roll {
    sides: u32,
    repeat: u32,
}

impl Roll {
    fn new(s: &str) -> Result<Roll, String> {
        let parts: Vec<&str> = s.split('d').collect();

        if parts.len() != 2 {
            Err("Not properly formatted".to_owned()) //TODO this is lazy
        } else {
            Ok(Roll {
                sides: u32::from_str(parts[1]).unwrap(),
                repeat: u32::from_str(parts[0]).unwrap(),
            })
        }
    }
}

#[derive(Debug)]
struct Outcome {
    rolls: Vec<u32>,
}

impl Outcome {
    fn new(roll: Roll) -> Outcome {
        let mut rolls = Vec::new();
        for _ in 1..(roll.repeat + 1) {
            let result = rand::thread_rng().gen_range(1, roll.sides + 1);
            rolls.push(result);
        }
        Outcome { rolls: rolls }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}\n{:?}",
            self.rolls.iter().fold(0, |acc, x| x + acc),
            self.rolls
        )
    }
}

fn roll(s: &str) -> Outcome {
    Outcome::new(Roll::new(s).unwrap())
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let result = roll(&arg1);
        println!("{}", result)
    }
}
