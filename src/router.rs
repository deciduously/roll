use gotham::router::Router;
use gotham::router::builder::*;
use handlers::*;

pub fn router() -> Router {
    build_simple_router(|route| {
        // can also use route.request(vec![Get, Head], "/")
        route.get_or_head("/").to(index);
        route.get("/roll").to(roll::index);
    })
}
