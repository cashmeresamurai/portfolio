#[macro_use]
extern crate rocket;
mod templating;
use templating::{about_me, index, static_files};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount("/", routes![index, about_me, static_files])
}
