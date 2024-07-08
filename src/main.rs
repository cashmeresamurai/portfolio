use askama_rocket::Template;
#[macro_use]
extern crate rocket;

use include_dir::{include_dir, Dir};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

// Einbinden des Verzeichnisses zur Kompilierzeit
static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

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
        .mount("/", routes![hello, static_files])
}
