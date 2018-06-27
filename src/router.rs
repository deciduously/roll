use actix_web::{App, http};
use handlers::*;

pub fn router() -> App {
    App::new()
        .route("/", http::Method::GET, index)
        .route("/roll/{tail:.*}", http::Method::GET, roll)
        //route
        //    .get("/roll/*")
        //    .with_path_extractor::<roll::PathExtractor>()
        //    .to(roll::index);
        // TODO this should be a single /items GET with QueryExtractor
        //route
        //    .get("/items")
        //    .with_path_extractor::<item::PathExtractor>() // ACTUALLY queryextractor
        //    .to(item::index);
        //route.post("/items").to(item::new_item);

        // don't forget about route.scope and associate - useful for when you implement the item stuff
}
