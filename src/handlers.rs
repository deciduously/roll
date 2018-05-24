use gotham::{http::response::create_response, state::{FromState, State}};
use hyper::{Response, StatusCode};
use mime;
use roll::roll::{Outcome, Roll};

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
        roll: String,
    }
    // the CLJS frontend will chain calls to this together, I think
    pub fn index(state: State) -> (State, Response) {
        let res = {
            let roll = PathExtractor::borrow_from(&state);
            create_response(
                &state,
                StatusCode::Ok,
                Some((
                    format!(
                        "Roll: {}\n{}",
                        roll.roll,
                        Outcome::new(&Roll::new(&roll.roll).unwrap())
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
