extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;
extern crate env_logger;
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
pub mod schema;

use actix_web::{http, middleware, server::HttpServer, App};
use handlers::*;
use roll::roll_strs;
use std::{
    env, io::{self, BufRead},
};

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
    // Create Actix system
    let sys = actix::System::new("roll");

    // grab env
    let env_addr = addr("127.0.0.1", dotenv!("PORT"));

    // init logger
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // define and start server
    println!("Listening for requests at http://{}", env_addr);
    HttpServer::new(move || {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(
                middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"),
            )
            .route("/", http::Method::GET, index)
            .route("/roll/{tail:.*}", http::Method::GET, roll)
            .resource("/items", |r| r.method(http::Method::GET).with(items))
            .resource("/item", |r| r.method(http::Method::POST).with(new_item))
    }).bind(env_addr)
        .unwrap()
        .start();

    //run actix
    let _ = sys.run();
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
