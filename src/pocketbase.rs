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
    println!("{}", &authorization);
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
    println!("{:#?}", &response);
    let response_data: ResponseData<T> = response.json().await?;

    Ok(response_data.items)
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoginStruct {
//     identity: String,
//     password: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct ResponseToken {
//     token: String,
// }

// pub async fn login_user() -> Result<()> {
//     let client: Client = Client::new();

//     let mut headers: HeaderMap = HeaderMap::new();

//     let authorization: String = env::var("AUTHORIZATION").expect("could not get authorization key");

//     headers.insert("Authorization", HeaderValue::from_str(&authorization)?);

//     let login: LoginStruct = LoginStruct {
//         identity: "mur1chan".to_string(),
//         password: "1HI7o7Naxh1bELr1".to_string(),
//     };
//     let response = client
//         .post("https://pocketbase.sakura.pm/api/collections/users/auth-with-password")
//         .headers(headers)
//         .json(&login)
//         .send()
//         .await?;

//     // Debugging-Druck der tatsächlichen Antwort
//     let response_text = response.text().await?;
//     println!("Response: {}", response_text);

//     // Anpassen der Struktur, falls notwendig
//     let response_data: ResponseToken = serde_json::from_str(&response_text)?;

//     // println!("Token: {}", response_data.token);

//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio;

//     #[tokio::test]
//     async fn test_login() {
//         let result = login_user().await;
//         println!("{:#?}", result)
//     }
// }
