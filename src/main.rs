#[macro_use]
extern crate rocket;
mod templating;
use templating::{
    about_me, index, project_details, projects, return_matrix_client, return_matrix_server,
    static_files, submit_task,
};
mod pocketbase;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));

        // X-Frame-Options entfernen oder anpassen
        response.remove_header("X-Frame-Options");

        // CSP für Embedding erlauben
        response.set_header(Header::new(
            "Content-Security-Policy",
            "frame-ancestors 'self' *;",
        ));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 3949)),
        )
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                about_me,
                static_files,
                projects,
                project_details,
                submit_task,
                return_matrix_server,
                return_matrix_client
            ],
        )
}
