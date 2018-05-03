// No args, a REPL-like environemnt which spits out your rolls.
// With an arg, just outputs

extern crate rand;

use std::{env, fmt, str::FromStr};
use rand::Rng;

#[derive(Debug)]
struct Roll {
    sides: u32,
    repeat: u32,
}

#[derive(Debug)]
struct Outcome {
    total: u32,
    rolls: Vec<u32>
}

impl Outcome {
    fn new(total: u32, rolls: Vec<u32>) -> Outcome {
        Outcome { total, rolls }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n{:?}", self.total, self.rolls)
    }
}

fn parse_roll(s: &str) -> Result<Roll, String> {
    let parts: Vec<&str> = s.split('d').collect();

    if parts.len() != 2 {
        Err("Not properly formatted".to_owned())
    } else {
        Ok(Roll {
            sides: u32::from_str(parts[1]).unwrap(),
            repeat: u32::from_str(parts[0]).unwrap(),
        })
    }
}

fn execute_roll(r: Roll) -> Outcome {
    let mut ret = Vec::new();
    for _ in 1..(r.repeat + 1) {
        let res = rand::thread_rng().gen_range(1, r.sides + 1);
        ret.push(res);
    }
    let total = ret.iter().fold(0, |acc, x| x + acc);
    Outcome::new(total, ret)
}

// TODO return a struct with the total and each roll
// and impl display
fn roll(s: &str) -> Outcome {
    execute_roll(parse_roll(s).unwrap())
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let result = roll(&arg1);
        println!("{}", result)
    }
}
