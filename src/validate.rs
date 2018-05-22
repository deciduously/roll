use regex::Regex;
use roll::Roll;
use std::{io, str::FromStr};

// check input type:
// XdX is a roll
// a single number is a multiplier followed by a roll: 8 1d4
// any other string is a lookup - for now just an item

#[derive(Debug)]
pub enum Command {
    Roll(Vec<Roll>),              // One or more XdX args
    Multiplier(u32, Vec<String>), // an integer repeater, and then either rolls or lookups
    Lookup(String),               // we get the roll from a file, there shouldn't be anything else
}

pub fn validate_input(s: &[String]) -> io::Result<Command> {
    lazy_static! {
        static ref ROLL_RE: Regex = Regex::new(r"^(?P<r>\d)d(?P<s>\d)").unwrap();
        static ref MULT_RE: Regex = Regex::new(r"^(?P<m>\d)").unwrap();
    }

    if ROLL_RE.is_match(&s[0]) {
        let mut ret = Vec::new();
        for arg in s {
            ret.push(Roll::new(arg)?);
        }
        return Ok(Command::Roll(ret));
    } else if MULT_RE.is_match(&s[0]) {
        if s[1..].len() >= 1 {
            return Ok(Command::Multiplier(
                u32::from_str(&s[0]).unwrap(),
                s[1..].to_vec(),
            ));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Need something to multiply!",
            ));
        }
    } else {
        if s.len() == 1 {
            return Ok(Command::Lookup(s[0].clone()));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Only one lookup at a time, please",
            ));
        }
    }
}

//you should write some tests
