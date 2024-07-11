use crate::db_query::{fetch_about_me, AboutMe};
use askama_rocket::Template;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use rocket_json_response::{
    serialize_to_json, JSONResponse, JSONResponseCode, JSONResponseWithoutData,
};
use serde::Serialize;
use std::path::PathBuf;
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[rocket::get("/")]
pub fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "about_me.html")]
pub struct AboutMeTemplate {
    about_me_entries: Vec<AboutMe>,
}

#[rocket::get("/about-me")]
pub async fn about_me() -> AboutMeTemplate {
    let about_me_entries = fetch_about_me().await;
    AboutMeTemplate {
        about_me_entries: about_me_entries.expect("could not unpack "),
    }
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate;

#[rocket::get("/projects")]
pub async fn projects() -> ProjectsTemplate {
    ProjectsTemplate
}
static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[rocket::get("/static/<path..>", rank = 100)]
pub async fn static_files(path: PathBuf) -> Option<(ContentType, Vec<u8>)> {
    let path_str = path.to_string_lossy();
    let data = PROJECT_DIR.get_file(&*path_str)?;

    let content_type = path
        .extension()
        .and_then(|e| e.to_str())
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Binary);

    Some((content_type, data.contents().to_vec()))
}

#[derive(Serialize)]
struct MatrixClient {
    #[serde(rename = "m.homeserver")]
    homeserver: HomeServer,
}
serialize_to_json!(MatrixClient);

#[derive(Serialize)]
struct HomeServer {
    base_url: String,
}

#[derive(Serialize)]
struct MatrixServer {
    #[serde(rename = "m.server")]
    server: String,
}
serialize_to_json!(MatrixServer);

#[rocket::get("/.well-known/matrix/client")]
pub async fn return_matrix_client() -> JSONResponse<'static, MatrixClient> {
    let response = MatrixClient {
        homeserver: HomeServer {
            base_url: "https://matrix.sakura.pm".to_string(),
        },
    };

    JSONResponse::ok(response)
}

#[rocket::get("/.well-known/matrix/server")]
pub async fn return_matrix_server() -> JSONResponse<'static, MatrixServer> {
    let response = MatrixServer {
        server: "matrix.sakura.pm:8448".to_string(),
    };

    JSONResponse::ok(response)
}
