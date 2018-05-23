use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;

// this signature is the gotham Handler trait
pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Oh hey there").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

//pub fn roll_handler
