use askama_rocket::Template;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use std::path::PathBuf;
// index template
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[rocket::get("/")]
pub fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "about_me.html")]
pub struct AboutMeTemplate;
#[rocket::get("/about-me")]
pub fn about_me() -> AboutMeTemplate{
    AboutMeTemplate
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
