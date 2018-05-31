use gotham::router::{builder::*, Router};
use handlers::*;

pub fn router() -> Router {
    build_simple_router(|route| {
        // can also use route.request(vec![Get, Head], "/")
        route.get_or_head("/").to(index);
        route
            .get("/roll/*")
            .with_path_extractor::<roll::PathExtractor>()
            .to(roll::index);
        // TODO this should be a single /items GET with QueryExtractor
        //route
        //    .get("/item/:item")
        //    .with_path_extractor::<item::PathExtractor>()
        //    .to(item::index);
        route.post("/items").to(item::new_item);

        // don't forget about route.scope and associate - useful for when you implement the item stuff
    })
}
