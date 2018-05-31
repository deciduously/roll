use gotham::{http::response::create_response,
             handler::{HandlerFuture, IntoHandlerError},
             state::{FromState, State}};
use hyper::{Body, Response, StatusCode};
use item::*;
use models::*;
use mime;
use db::*;
use roll::*;
use serde_json;
use futures::{future, Future, Stream};

// this signature is the gotham Handler trait
// This is going to be a microservice though - the frontend will be Clojure
// not sure I need this - think about it.
pub fn index(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("dice roller").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

header! { (AccessControl, "Access-Control-Allow-Origin") => [String] }

pub mod item {
    use super::*;

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct PathExtractor {
        item: String,
    }

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct QueryExtractor {
        title: String,
        damage: String,
    }

//    pub fn index(state: State) -> (State, Response) {
//        let mut res = {
//            let i = PathExtractor::borrow_from(&state);
//            let items = get_items().unwrap();
//            let ret = item::lookup_item(&i.item, &items).unwrap();
//            create_response(
//                &state,
//                StatusCode::Ok,
//                Some((
//                    serde_json::to_string(&Item {
//                        id: 
//                        name: ret.0,
//                        damage: ret.1,
//                    }).expect("serialized item")
//                        .into_bytes(),
//                    mime::APPLICATION_JSON,
//                )),
//            )
//        };
//
//        {
//            let headers = res.headers_mut();
//            headers.set(AccessControl("*".to_string()))
//        };
//        (state, res)
//    }

    pub fn new_item(mut state: State) -> Box<HandlerFuture> {
        // grab db connection
        // TODO r2d2

        // read POST data

        let f = Body::take_from(&mut state)
            .concat2()
            .then(|full_body| match full_body {
                Ok(valid_body) => {
                    let connection = establish_connection();
                    let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                    // try to add an item from
                    let new_item: NewItem = serde_json::from_str(&body_content).expect("properly formed POST body");
                    create_item(&connection, new_item.title, new_item.damage);
                    println!("Body: {}\nNewItem: {:?}", body_content, new_item);
                    let res = create_response(&state, StatusCode::Ok, None);
                    future::ok((state, res))
                }
                Err(e) => return future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }
}

pub mod roll {
    use super::*;

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct PathExtractor {
        #[serde(rename = "*")]
        cmds: Vec<String>,
    }

    pub fn index(state: State) -> (State, Response) {
        let mut res = {
            let cmd = PathExtractor::borrow_from(&state);
            let outcomes = roll_strs(&cmd.cmds);

            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    serde_json::to_string(&outcomes)
                        .expect("serialized outcome")
                        .into_bytes(),
                    mime::APPLICATION_JSON,
                )),
            )
        };

        {
            let headers = res.headers_mut();
            headers.set(AccessControl("*".to_string()))
        };
        (state, res)
    }
}

//pub fn roll_handler

// You should do a database instead of YAML files, and have a web form for additions instead of editing a YAML file
