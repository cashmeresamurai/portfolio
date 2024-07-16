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
struct Item {
    about_me: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData<T> {
    items: Vec<T>,
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
        .json::<ResponseData<_>>()
        .await?;

    let about_me_vec = res
        .items
        .into_iter()
        .map(|item: AboutMe| AboutMe {
            about_me: item.about_me,
        })
        .collect();

    Ok(about_me_vec)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectShowcase {
    pub id: String,
    pub project_slug: String,
    pub project_title: String,
    pub project_description: String,
    pub project_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseDataProject {
    items: Vec<ProjectShowcase>,
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
        .get(format!(
            "https://pocketbase.sakura.pm/api/collections/{}/records",
            &collection
        ))
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<ResponseDataProject>()
        .await?;

    let project_showcase_vec = res.items.into_iter().collect();

    Ok(project_showcase_vec)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDetails {
    pub id: String,
    pub title: String,
    pub images: Vec<String>,
    pub description: String,
    pub cards: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDataProjectDetails {
    pub items: Vec<ProjectDetails>,
}

pub async fn fetch_project_details(project_slug: &str) -> Result<Vec<ProjectDetails>> {
    // https://pocketbase.sakura.pm/api/collections/num63jj3oy768ue/records?page=1&perPage=40&sort=-created&skipTotal=1&filter=&expand=&fields=*%2Cproject_description%3Aexcerpt(200)
    let collection: String = String::from("num63jj3oy768ue");
    let client = Client::new();

    // Abrufen der Umgebungsvariablen zur Laufzeit
    let authorization = env::var("AUTHORIZATION").expect("could not get authorization key");

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let filter = format!("identifier='{}'", project_slug);
    let params = [
        ("page", "1"),
        ("perPage", "40"),
        ("sort", ""),
        ("skipTotal", "1"),
        ("filter", &filter),
        ("expand", ""),
        ("fields", ""),
    ];
    let res = client
        .get(format!(
            "https://pocketbase.sakura.pm/api/collections/{}/records",
            &collection
        ))
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<ResponseDataProjectDetails>()
        .await?;
    println!("{:#?}", res);
    let project_showcase_vec = res
        .items
        .first()
        .expect("could not get projectdetails struct");
    let mut vecc = vec![];
    vecc.push(project_showcase_vec.clone());
    Ok(vecc)
}
