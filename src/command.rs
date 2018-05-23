use regex::Regex;
use roll::{Outcome, Roll};
use std::{io, str::FromStr};

// check input type:
// XdX is a roll
// a single number is a multiplier followed by a roll: 8 1d4
// any other string is a lookup - for now just an item

#[derive(Debug, PartialEq)]
pub enum Command {
    Roll(Vec<Roll>),              // One or more XdX args
    Multiplier(u32, Vec<String>), // an integer repeater, and then either rolls or lookups
    Lookup(String),               // we get the roll from a file, there shouldn't be anything else
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Roll(rolls) => {
                for roll in rolls {
                    println!("{}", Outcome::new(roll));
                }
            }
            Command::Multiplier(times, input) => {
                let cmd = validate_input(input).unwrap();
                for _ in 0..*times {
                    cmd.run();
                }
            }
            Command::Lookup(s) => {
                println!("Looking up {}...BZZRT.  Lookup not implemented", s);
            }
        }
    }
}

pub fn validate_input(s: &[String]) -> io::Result<Command> {
    lazy_static! {
        static ref ROLL_RE: Regex = Regex::new(r"^\dd\d").unwrap();
        static ref MULT_RE: Regex = Regex::new(r"^\d").unwrap();
    }

    if ROLL_RE.is_match(&s[0]) {
        let mut ret = Vec::new();
        for arg in s {
            ret.push(Roll::new(arg)?);
        }
        Ok(Command::Roll(ret))
    } else if MULT_RE.is_match(&s[0]) {
        if !s[1..].is_empty() {
            Ok(Command::Multiplier(
                u32::from_str(&s[0]).unwrap(),
                s[1..].to_vec(),
            ))
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Need something to multiply!",
            ))
        }
    } else if s.len() == 1 {
        Ok(Command::Lookup(s[0].clone()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Only one lookup at a time, please",
        ))
    }
}

#[test]
fn test_single_roll_command() {
    assert_eq!(
        Command::Roll(vec![Roll::new("2d6").unwrap()]),
        validate_input(&vec!["2d6".to_string()]).unwrap()
    );
}

#[test]
fn test_multiple_rolls_command() {
    assert_eq!(
        Command::Roll(vec![Roll::new("2d6").unwrap(), Roll::new("1d10").unwrap()]),
        validate_input(&vec!["2d6".to_string(), "1d10".to_string()]).unwrap()
    );
}

#[test]
#[should_panic]
fn test_malformed_roll_command() {
    validate_input(&vec!["2d8".to_string(), "1dd".to_string()]).unwrap();
}

#[test]
fn test_mult_command() {
    assert_eq!(
        Command::Multiplier(2, vec!["2d6".to_string(), "1d4".to_string()]),
        validate_input(&vec!["2".to_string(), "2d6".to_string(), "1d4".to_string()]).unwrap()
    )
}

#[test]
fn test_lookup_command() {
    assert_eq!(
        Command::Lookup("blello".to_string()),
        validate_input(&vec!["blello".to_string()]).unwrap()
    );
}

#[test]
#[should_panic]
fn test_excess_lookups() {
    validate_input(&vec!["blello".to_string(), "mellow".to_string()]).unwrap();
}
