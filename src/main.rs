extern crate rand;

use std::{env, fmt, process, str::FromStr};
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

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.repeat, self.sides)
    }
}

#[derive(Debug)]
struct Outcome {
    roll: Roll,
    rolls: Vec<u32>,
}

impl Outcome {
    fn new(roll: Roll) -> Outcome {
        let mut rolls = Vec::new();
        for _ in 1..(roll.repeat + 1) {
            let result = rand::thread_rng().gen_range(1, roll.sides + 1);
            rolls.push(result);
        }
        Outcome { roll, rolls: rolls }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:\n{:?}\n{:?}\n",
            self.roll,
            self.rolls.iter().fold(0, |acc, x| x + acc),
            self.rolls
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        //TODO make a REPL-ish thing
        println!("No roll provided!");
        process::exit(1);
    }

    let rolls: Vec<Roll> = args[1..].iter().map(|i| Roll::new(i).unwrap()).collect();

    for roll in rolls {
        println!("{}", Outcome::new(roll))
    }
}
