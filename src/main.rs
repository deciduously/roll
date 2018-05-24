extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate roll;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod handlers;
mod router;

use roll::command::validate_input;
use router::router;
use std::{env, io::{self, BufRead}};

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
    gotham::start(addr, router())
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
    use hyper::StatusCode;

    #[test]
    fn index_get_test() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"dice roller");
    }

    #[test]
    fn index_head_test() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .head("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);
        assert!(response.read_body().unwrap().is_empty());
    }

    #[test]
    fn index_delete_test() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .delete("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::MethodNotAllowed);
    }

    #[test]
    fn roll_is_extracted_test() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/roll/1d6")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Roll: 1d6");
    }
}
