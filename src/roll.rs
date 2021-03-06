use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use command::validate_input;
use rand::{self, Rng};
use serde_json;
use std::{fmt, io, str::FromStr};

#[derive(Debug, PartialEq, Serialize)]
pub struct Roll {
    sides: u32,
    repeat: u32,
}

impl Roll {
    pub fn new(s: &str) -> io::Result<Roll> {
        // Note - we've already validated this in command.rs
        // Should I check again here?
        let parts: Vec<&str> = s.split('d').collect();

        Ok(Roll {
            sides: u32::from_str(parts[1]).unwrap(),
            repeat: u32::from_str(parts[0]).unwrap(),
        })
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.repeat, self.sides)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Outcome {
    roll: String,
    rolls: Vec<u32>,
}

impl Outcome {
    pub fn new(roll: &Roll) -> Outcome {
        let mut rolls = Vec::new();
        for _ in 1..(roll.repeat + 1) {
            let result = rand::thread_rng().gen_range(1, roll.sides + 1);
            rolls.push(result);
        }
        Outcome {
            roll: roll.to_string(),
            rolls,
        }
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

#[derive(Serialize)]
pub struct Outcomes {
    pub outcomes: Vec<Outcome>,
}

impl Responder for Outcomes {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub fn roll_strs(s: &[String]) -> Outcomes {
    validate_input(s).unwrap().run()
}

#[test]
fn test_valid_roll() {
    assert_eq!(
        Roll::new("2d6").unwrap(),
        Roll {
            sides: 6,
            repeat: 2
        }
    )
}

#[test]
#[should_panic]
fn test_invalid_roll() {
    Roll::new("2d").unwrap();
}
