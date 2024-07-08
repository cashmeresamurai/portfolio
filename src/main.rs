use askama_rocket::Template;
#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[rocket::get("/")]
fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/static", FileServer::from(relative!("/static/")))
}
