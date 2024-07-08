use askama_rocket::Template;
#[macro_use]
extern crate rocket;

use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[rocket::get("/")]
fn index() -> IndexTemplate{
    IndexTemplate
}

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[rocket::get("/static/<path..>", rank = 100)]
async fn static_files(path: PathBuf) -> Option<(ContentType, Vec<u8>)> {
    let path_str = path.to_string_lossy();
    let data = PROJECT_DIR.get_file(&*path_str)?;

    let content_type = path
        .extension()
        .and_then(|e| e.to_str())
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Binary);

    Some((content_type, data.contents().to_vec()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount("/", routes![index, static_files])
}
