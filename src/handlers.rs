use actix_web::{HttpRequest, Json, Responder, Result};
use db::DB_POOL;
use item::{create_item, get_items, RequestItem};
use roll::*;

// GET /
pub fn index(_req: HttpRequest) -> &'static str {
    "dice roller"
}

// GET /roll/{cmd}
pub fn roll(req: HttpRequest) -> impl Responder {
    let cmd = &req.match_info()["tail"];
    println!("cmd: {}", cmd);
    // Is there a better way?
    let cmds = ((&cmd)
        .split('/')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string()))
        .collect::<Vec<String>>();
    roll_strs(&cmds)
}

// GET /items
pub fn items(_req: HttpRequest) -> impl Responder {
    let conn = DB_POOL.get().expect("Could not get DB connection");
    let ret = get_items(&conn);
    println!("{:?}", ret);
    ret
}

// POST /item
pub fn new_item(item: Json<RequestItem>) -> Result<String> {
    // ideally if it already exists, we instead just update the existing record
    // but lets not get ahead of oureselves

    let conn = DB_POOL.get().expect("Could not get DB connection");
    let s = create_item(&conn, &item.name, &item.damage);
    println!("Creating item: {:?}", item);
    // We should really be returning the created item
    Ok(format!("Ok! size: {}", s))
}
