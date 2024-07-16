#[macro_use]
extern crate rocket;
mod templating;
use templating::{
    about_me, index, project_details, projects, return_matrix_client, return_matrix_server,
    static_files,
};
mod pocketbase;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .mount(
            "/",
            routes![
                index,
                about_me,
                static_files,
                projects,
                project_details,
                return_matrix_server,
                return_matrix_client
            ],
        )
}
