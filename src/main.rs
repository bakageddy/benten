use std::{
    collections::HashMap,
    io::{self, stdout, BufRead, Write},
};

use serde::Deserialize;

const BASE_URL: &'static str = "https://api.mangadex.org";

struct SearchRequest<'a> {
    query: &'a str,
    limit: i32,
    offset: i32,
}

impl<'a> SearchRequest<'a> {
    fn new(query: &'a str, limit: i32, offset: i32) -> Self {
        Self {
            query,
            limit,
            offset,
        }
    }

    async fn get(&self, client: &reqwest::Client) -> anyhow::Result<SearchResult> {
        let target_url = format!("{BASE_URL}/manga");

        let mut params = HashMap::new();
        params.insert("title", self.query.to_string());
        params.insert("limit", self.limit.to_string());
        params.insert("offset", self.offset.to_string());
        // params.insert("order[latestUploadedChapter]", String::from("asc"));

        let res = client.get(target_url).query(&params).send().await?;
        let search_result = res.json().await?;
        Ok(search_result)
    }
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    data: Vec<MangaInfo>,
}

#[derive(Debug, Deserialize)]
struct MangaInfo {
    id: String,
    attributes: MangaAttributes,
}

impl MangaInfo {
    fn download_chapter(&self, chapter_id: String, client: reqwest::Client) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct MangaAttributes {
    title: Title,
}

#[derive(Debug, Deserialize)]
enum Title {
    #[serde(rename = "en")]
    English(String),
    #[serde(rename = "ja-ro")]
    JapaneseRomanized(String),
}

#[derive(Debug)]
struct ChapterSearchRequest<'a> {
    manga_id: &'a str,
    limit: i32,
    offset: i32,
}

impl<'a> ChapterSearchRequest<'a> {
    fn new(manga_id: &'a str, limit: i32, offset: i32) -> Self {
        Self { manga_id , limit, offset }
    }

    async fn get(&self, client: &reqwest::Client) -> anyhow::Result<ChapterSearchResponse> {
        let url = format!("{BASE_URL}/manga/{id}/feed", id = self.manga_id);
        let mut params = HashMap::new();
        let limit = self.limit.to_string();
        params.insert("limit", limit);
        params.insert("includeExternalUrl", String::from("0"));
        params.insert("order[chapter]", String::from("asc"));
        params.insert("translatedLanguage[]", String::from("en"));
        let res = client.get(url).query(&params).send().await?;
        let feed_response = res.json().await?;
        Ok(feed_response)
    }
}

#[derive(Debug, Deserialize)]
struct ChapterSearchResponse {
    data: Vec<ChapterInfo>,
}

#[derive(Debug, Deserialize)]
struct ChapterInfo {
    id: String,
    #[serde(rename="attributes")]
    attr: ChapterAttributes,
}

impl ChapterInfo {
    fn download(&self, client: reqwest::Client) -> anyhow::Result<()> {
        todo!()
        // Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct ChapterAttributes {
    volume: String,
    chapter: String,
    pages: u32,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new().user_agent("Benten").build()?;
    let mut stdin = io::stdin().lock();
    let mut search_term = String::new();
    print!("Query: ");
    stdout().flush()?;
    stdin.read_line(&mut search_term)?;
    let search_request = SearchRequest::new(&search_term, 10, 0);
    let response = search_request.get(&client).await?;

    let mut index = 0;
    for manga in &response.data {
        println!("{index}: {:?}", manga.attributes.title);
        index += 1;
    }

    print!("Select from options: ");
    stdout().flush()?;

    let mut option = String::new();
    stdin.read_line(&mut option)?;
    let option: u32 = option.trim().parse()?;

    if let Some(manga) = response.data.get(option as usize) {
        let chapter_req = ChapterSearchRequest::new(&manga.id, 10, 0);
        let res = chapter_req.get(&client).await?;
        println!("{res:#?}");
    } else {
    }
    Ok(())
}
