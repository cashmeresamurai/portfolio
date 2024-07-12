use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutMe {
    pub entry_title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    entry_title: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    items: Vec<Item>,
}

pub async fn fetch_about_me() -> Result<Vec<AboutMe>> {
    let client = Client::new();

    // Abrufen der Umgebungsvariablen zur Laufzeit
    let authorization = env::var("AUTHORIZATION").expect("could not get authorization key");

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let params = [
        ("page", "1"),
        ("perPage", "40"),
        ("sort", "-created"),
        ("skipTotal", "1"),
        ("filter", ""),
        ("expand", ""),
        ("fields", ""),
    ];

    let res = client
        .get("https://pocketbase.sakura.pm/api/collections/mlkl9ra6e6ts7jt/records")
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<ResponseData>()
        .await?;

    let about_me_vec = res
        .items
        .into_iter()
        .map(|item| AboutMe {
            entry_title: item.entry_title,
            content: item.content,
        })
        .collect();

    Ok(about_me_vec)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectShowcase {
    pub project_slug: String,
    pub project_title: String,
    pub project_description: String,
    pub project_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemProject {
    id: String,
    project_slug: String,
    project_title: String,
    project_description: String,
    project_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseDataProject {
    items: Vec<ItemProject>,
}
pub async fn fetch_projects() -> Result<Vec<ProjectShowcase>> {
    let client = Client::new();

    // Abrufen der Umgebungsvariablen zur Laufzeit
    let authorization = env::var("AUTHORIZATION").expect("could not get authorization key");

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let params = [
        ("page", "1"),
        ("perPage", "40"),
        ("sort", "created"),
        ("skipTotal", "1"),
        ("filter", ""),
        ("expand", ""),
        ("fields", ""),
    ];
    let collection: String = String::from("z6b7hsoqbkmfwmi");
    let res = client
        .get(format!("https://pocketbase.sakura.pm/api/collections/{}/records", &collection))
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<ResponseDataProject>()
        .await?;

    let project_showcase_vec = res
        .items
        .into_iter()
        .map(|item| ProjectShowcase {
            
            project_slug: item.project_slug,
            project_title: item.project_title,
            project_description: item.project_description,
            project_image: format!("https://pocketbase.sakura.pm/api/files/{}/{}/{}", collection, item.id, item.project_image)
        })
        .collect();

    Ok(project_showcase_vec)
}
