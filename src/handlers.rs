use actix_web::{HttpRequest, Responder};
use db::DB_POOL;
use item::get_items;
use roll::*;

// GET /
pub fn index(_req: HttpRequest) -> &'static str {
    "dice roller"
}

//header! { (AccessControl, "Access-Control-Allow-Origin") => [String] } // TODO in actix!
// its middleware, see other Actix project

// GET roll/*
// GET items
// POST item {:name blah :damage blah} (or you know, in json)

// GET roll/*
pub fn roll(req: HttpRequest) -> impl Responder {
    let cmd = &req.match_info()["tail"];
    println!("cmd: {}", cmd);
    let cmds = ((&cmd).split("/").collect::<Vec<&str>>().iter().map(|s| s.to_string())).collect::<Vec<String>>();
    roll_strs(&cmds)
}

// GET items
pub fn items(req: HttpRequest) -> impl Responder {
    let conn = DB_POOL.get().unwrap();
    get_items(&conn)
}
