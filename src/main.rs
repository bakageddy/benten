#[derive(Debug, serde::Deserialize)]
enum ContentRating {
    safe,
    suggestive,
    erotica,
    pornographic
}

#[derive(Debug, serde::Deserialize)]
enum Title {
    en(String),
    ja(String),
    ja_ro(String),
}

#[derive(Debug, serde::Deserialize)]
struct Description {
    en: Option<String>,
    ru: Option<String>,
    ja: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Attributes {
    title: Title,
    description: Description,
}
#[derive(Debug, serde::Deserialize)]
struct Manga {
    id: String,
    attributes: Attributes,
}

#[derive(Debug, serde::Deserialize)]
struct SearchResponse {
    data: Vec<Manga>,
    total: i64,
}

use std::fs::{File, self};

use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &'static str = "https://api.mangadex.org";
    let client = match ClientBuilder::new().user_agent("benten/0.1").build() {
        Ok(c) => c,
        Err(_) => panic!("Failed to build client!"),
    };

    let req_url = format!("{}/manga?title={}", BASE_URL, "isekai");

    let res = match client.get(req_url).send().await {
        Ok(x) => x,
        Err(_) => panic!("Failed to get response!"),
    };

    // File::create("./output.txt");
    // fs::write("./output.txt", res.text().await?);
    
    println!("{:#?}", res.json::<SearchResponse>().await?);
    Ok(())
}
