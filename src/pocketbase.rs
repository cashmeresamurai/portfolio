use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutMe {
    pub about_me: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData<T> {
    items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectShowcase {
    pub id: String,
    pub project_slug: String,
    pub project_title: String,
    pub project_description: String,
    pub project_image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDetails {
    pub id: String,
    pub title: String,
    pub images: Vec<String>,
    pub description: String,
    pub cards: String,
}

pub async fn get_collection<T>(collection: &str, filter: Option<&str>) -> Result<Vec<T>>
where
    for<'de> T: Deserialize<'de>,
{
    let client = Client::new();
    let filter: &str = filter.unwrap_or("");
    println!("{}", &filter);
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
        ("filter", filter),
        ("expand", ""),
        ("fields", ""),
    ];

    let response = client
        .get(format!(
            "https://pocketbase.sakura.pm/api/collections/{}/records",
            collection
        ))
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    let response_data: ResponseData<T> = response.json().await?;

    Ok(response_data.items)
}
