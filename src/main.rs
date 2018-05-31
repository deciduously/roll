#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod command;
pub mod db;
mod handlers;
pub mod item;
pub mod models;
pub mod roll;
mod router;
pub mod schema;

use roll::roll_strs;
use router::router;
use std::{env,
          io::{self, BufRead}};

lazy_static! {
    static ref DB_POOL: db::Pool = db::init_pool();
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

fn addr(root: &str, port: &str) -> String {
    format!("{}:{}", root, port)
}

fn server() {
    let port = dotenv!("PORT");
    let root = "127.0.0.1";

    let addr = addr(root, port);
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
    } else {
        // Otherwise simply try to parse the args given as a command
        roll_strs(&args[1..]);
    }
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
    }
}
