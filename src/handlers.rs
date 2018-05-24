use gotham::{http::response::create_response, state::{FromState, State}};
use hyper::{Response, StatusCode};
use mime;

// this signature is the gotham Handler trait
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
        roll: String, // this will eventually be the whole command?  or something?
    }

    pub fn index(state: State) -> (State, Response) {
        let res = {
            let roll = PathExtractor::borrow_from(&state);
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    format!("Roll: {}", roll.roll).into_bytes(),
                    mime::TEXT_PLAIN,
                )),
            )
        };

        (state, res)
    }
}

//pub fn roll_handler

// You should do a database instead of YAML files, and have a web form for additions instead of editing a YAML file
