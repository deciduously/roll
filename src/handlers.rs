use db::*;
use futures::{future, Future, Stream};
use gotham::{handler::{HandlerFuture, IntoHandlerError},
             http::response::create_response,
             state::{FromState, State}};
use hyper::{Body, Response, StatusCode};
use item::*;
use mime;
use models::*;
use roll::*;
use serde_json;

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

    pub fn index(state: State) -> (State, Response) {
        let mut res = {
            //let i = PathExtractor::borrow_from(&state);
            let connection = DB_POOL
                .get()
                .expect("Could not get DB conn from thread pool");
            let items = get_items(&connection);
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    serde_json::to_string(&items)
                        .expect("serialized items")
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

    pub fn new_item(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(|full_body| match full_body {
                Ok(valid_body) => {
                    let connection = DB_POOL
                        .get()
                        .expect("Could not get DB conn from thread pool");
                    let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                    println!("Body: {}", body_content);
                    // try to add an item from
                    let new_item: NewItem =
                        serde_json::from_str(&body_content).expect("properly formed POST body");
                    // TODO if item already exists, just update instead
                    create_item(&connection, new_item.title, new_item.damage); // TODO write a fn to take a NewItem instead
                    println!("NewItem: {:?}", new_item);
                    let mut res = create_response(&state, StatusCode::Ok, None);
                    {
                        let headers = res.headers_mut();
                        println!("Doing the access contol set on  POST");
                        headers.set(AccessControl("*".to_string()))
                    };
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
