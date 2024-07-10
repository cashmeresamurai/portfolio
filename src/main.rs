#[macro_use]
extern crate rocket;
mod templating;
use templating::{about_me, index, static_files};
mod db_query;
use db_query::init_db;
#[launch]
fn rocket() -> _ {
    match init_db() {
        Ok(()) => println!("db initialized successful"),
        Err(e) => println!("Error {}", e),
    }
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount("/", routes![index, about_me, static_files])
}
