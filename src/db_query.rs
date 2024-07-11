use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue};
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
    let env = &env::var("Authorization").expect("could not get authorization key");
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Authorization", HeaderValue::from_static(authorization));
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
        .get("http://pocketbase-ikg0cog.95.217.222.184.sslip.io/api/collections/mlkl9ra6e6ts7jt/records")
        .headers(headers)
        .query(&params)
        .send()
        .await?
        .json::<ResponseData>()
        .await?;

    let about_me_vec = res.items.into_iter().map(|item| AboutMe {
        entry_title: item.entry_title,
        content: item.content,
    }).collect();

    Ok(about_me_vec)
}
