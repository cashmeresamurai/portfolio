use askama_rocket::Template;
#[macro_use]
extern crate rocket;

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
        .configure(rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 3949))
        )
        .mount("/", routes![hello])
}
