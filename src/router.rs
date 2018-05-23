use gotham::router::Router;
use gotham::router::builder::*;
use handlers::*;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(say_hello);
    })
}
