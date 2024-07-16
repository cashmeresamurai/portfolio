use crate::db_query::{
    fetch_about_me, fetch_project_details, fetch_projects, AboutMe, ProjectDetails, ProjectShowcase,
};
use askama_rocket::Template;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use rocket::response::content::RawJson;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[rocket::get("/")]
pub async fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "project_details.html")]
pub struct ProjectDetailsTemplate {
    pub projects_showcase_vec: Vec<ProjectDetails>,
}

#[rocket::get("/project-details/<project_slug>")]
pub async fn project_details(project_slug: &str) -> ProjectDetailsTemplate {
    let project_details = fetch_project_details(project_slug).await;
    ProjectDetailsTemplate {
        projects_showcase_vec: project_details
            .expect("could not fetch Project Showcase Entries from Pocketbase"),
    }
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
        about_me_entries: about_me_entries
            .expect("could not fetch About Me Entries from Pocketbase"),
    }
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    projects_showcase_vec: Vec<ProjectShowcase>,
}

#[rocket::get("/projects")]
pub async fn projects() -> ProjectsTemplate {
    // fetch_project_details().await;
    let projects_vec = fetch_projects().await;
    println!("{:#?}", projects_vec);
    ProjectsTemplate {
        projects_showcase_vec: projects_vec
            .expect("could not fetch Project Showcase Entries from Pocketbase"),
    }
}

#[derive(Template)]
#[template(path = "modal.html")]
pub struct ModalTemplate;

#[rocket::get("/modal")]
pub fn modal() -> ModalTemplate {
    ModalTemplate
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
pub struct MatrixClient {
    #[serde(rename = "m.homeserver")]
    pub homeserver: HomeServer,
}

#[derive(Serialize)]
pub struct HomeServer {
    pub base_url: String,
}

#[derive(Serialize)]
pub struct MatrixServer {
    #[serde(rename = "m.server")]
    server: String,
}

#[rocket::get("/.well-known/matrix/client")]
pub async fn return_matrix_client() -> RawJson<String> {
    let response = MatrixClient {
        homeserver: HomeServer {
            base_url: "https://matrix.sakura.pm".to_string(),
        },
    };
    let json_response = serde_json::to_string(&response).unwrap();
    RawJson(json_response)
}

#[rocket::get("/.well-known/matrix/server")]
pub async fn return_matrix_server() -> RawJson<String> {
    let response = MatrixServer {
        server: "matrix.sakura.pm:8448".to_string(),
    };
    let json_response = serde_json::to_string(&response).unwrap();
    RawJson(json_response)
}
