#[macro_use]
extern crate rocket;
mod templating;
use templating::{about_me, index, static_files};
mod db_query;
use db_query::{db_query, return_collection};
#[launch]
fn rocket() -> _ {
    // match db_query() {
    //     Ok(()) => println!("db created successful"),
    //     Err(e) => println!("Error {}", e),
    // }
    // match return_collection() {
    //     Ok(collection) => {
    //         println!("Collection returned successfully: {:?}", collection);
    //     },
    //     Err(e) => {
    //         println!("Error returning collection: {}", e);
    //     }
    // }
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount("/", routes![index, about_me, static_files])
}
