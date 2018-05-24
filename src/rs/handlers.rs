use gotham::{http::response::create_response, state::{FromState, State}};
use hyper::{Response, StatusCode};
use mime;
use roll::roll::*;

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

pub mod roll {
    use super::*;

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct PathExtractor {
        #[serde(rename = "*")]
        cmds: Vec<String>,
    }

    pub fn index(state: State) -> (State, Response) {
        let res = {
            let cmd = PathExtractor::borrow_from(&state);
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    format!(
                        "Command: {:?}\n{}",
                        cmd.cmds,
                        roll_strs(&cmd.cmds) // TODO better error handling - just throws a 500
                    ).into_bytes(),
                    mime::TEXT_PLAIN,
                )),
            )
        };

        (state, res)
    }
}

//pub fn roll_handler

// You should do a database instead of YAML files, and have a web form for additions instead of editing a YAML file
