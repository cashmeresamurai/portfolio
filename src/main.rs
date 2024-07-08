use askama_rocket::Template;
#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");
#[rocket::get("/")]
fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

// Handler für statische Dateien
#[rocket::get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(&file);
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount("/", routes![hello])
        .mount("/static", FileServer::from(relative!("static/")))
}
