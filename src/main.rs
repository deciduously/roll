extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate roll;

use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use roll::command::validate_input;
use std::{env, io::{self, BufRead}};

// this signature is the gotham Handler trait
pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Oh hey there").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

fn roll_strs(s: &[String]) {
    validate_input(s).unwrap().run();
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

fn server() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // REPL mode when called with no arguments
    if args.len() <= 1 {
        repl();
    }

    // Server mode called via `roll serve`
    // TODO what to do with any trailing args?
    if args[1] == "serve" {
        server();
    }

    // Otherwise simply try to parse the args given as a command
    roll_strs(&args[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Oh hey there");
    }
}
