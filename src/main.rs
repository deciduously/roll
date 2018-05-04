extern crate roll;

use roll::{Roll, Outcome};
use std::{env, io::{self, BufRead}};

fn roll_strs(s: &[String]) {
    let rolls: Vec<Roll> = s.iter().map(|i| Roll::new(i).unwrap()).collect();

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
        let each: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect(); // EXPENSIVE, why reallocate
        println!("{:?}", each);

        roll_strs(&each);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
       repl();
    }

    roll_strs(&args[1..]);
}
