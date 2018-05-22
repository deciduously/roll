extern crate roll;

use roll::{parse::load_items, roll::{Outcome, Roll}};
use std::{env, io::{self, BufRead}};

enum Command {
    Roll(String),
    Multiplier(u32, Roll),
    Lookup(String),
}

fn validate_input(s: String) -> io::Result<Command> {
    Ok(Command::Roll(s))
}

fn roll_strs(s: &[String]) {
    let rolls: Vec<Roll> = s.iter().map(|i| Roll::new(i).unwrap()).collect();

    // check input type:
    // XdX is a roll
    // a single number is a multiplier followed by a roll: 8 1d4
    // any other string is a lookup - for now just an item

    for roll in rolls {
        println!("{}", Outcome::new(roll));
    }
}

fn repl() {
    println!("Use Ctrl-C to quit");
    loop {
        // TODO quit command
        println!("Roll: ");
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).unwrap();
        let each: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect(); // EXPENSIVE, why reallocate
        println!("{:?}", each);

        roll_strs(&each);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let items = load_items().unwrap();
    println!("{:?}", items);
    if args.len() <= 1 {
        repl();
    }

    roll_strs(&args[1..]);
}
