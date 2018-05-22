use rand::{self, Rng};
use std::{fmt, io, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Roll {
    sides: u32,
    repeat: u32,
}

impl Roll {
    pub fn new(s: &str) -> io::Result<Roll> {
        let parts: Vec<&str> = s.split('d').collect();

        Ok(Roll {
            sides: u32::from_str(parts[1]).unwrap(),
            repeat: u32::from_str(parts[0]).unwrap(),
        })
        //   }
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.repeat, self.sides)
    }
}

#[derive(Debug)]
pub struct Outcome {
    roll: Roll,
    rolls: Vec<u32>,
}

impl Outcome {
    pub fn new(roll: Roll) -> Outcome {
        let mut rolls = Vec::new();
        for _ in 1..(roll.repeat + 1) {
            let result = rand::thread_rng().gen_range(1, roll.sides + 1);
            rolls.push(result);
        }
        Outcome { roll, rolls }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}:\n{:?}\n{:?}",
            self.roll,
            self.rolls.iter().fold(0, |acc, x| x + acc),
            self.rolls
        )
    }
}
