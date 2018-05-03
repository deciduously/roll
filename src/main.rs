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

// TODO accept multiple rolls, roll them all!
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        //TODO make a REPL-ish thing
        println!("No roll provided!");
        process::exit(1);
    }

    let rolls: Vec<Roll> = args[1..].iter().map(|i| Roll::new(i).unwrap()).collect();

    println!("{:?}", rolls);

    if let Some(arg1) = env::args().nth(1) {
        let result = roll(&arg1);
        println!("{}", result)
    } else {
        println!("Usage: roll 1d6")
    }
}
