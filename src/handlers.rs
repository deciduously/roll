use gotham::{http::response::create_response,
             state::{FromState, State}};
use hyper::{Response, StatusCode};
use item::*;
use mime;
use parse::*;
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

    pub fn index(state: State) -> (State, Response) {
        let mut res = {
            let i = PathExtractor::borrow_from(&state);
            let items = load_items().unwrap(); // TODO this is also called for EACH Lookup cmd - gross
            let ret = lookup_item(&i.item, &items).unwrap();
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    serde_json::to_string(&RawItem {
                        name: ret.0,
                        damage: ret.1,
                    }).expect("serialized item")
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
