use genpdf::Alignment;
use serde::Deserialize;
use std::{collections::HashMap, io::Cursor, ops::Deref, path::Path};

pub const BASE_URL: &'static str = "https://api.mangadex.org";

pub struct SearchRequest<'a> {
    pub query: &'a str,
    pub limit: i32,
    pub offset: i32,
}

impl<'a> SearchRequest<'a> {
    pub fn new(query: &'a str, limit: i32, offset: i32) -> Self {
        Self {
            query,
            limit,
            offset,
        }
    }

    pub async fn get(&self, client: &reqwest::Client) -> anyhow::Result<SearchResult> {
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
pub struct SearchResult {
    pub data: Vec<MangaInfo>,
}

#[derive(Debug, Deserialize)]
pub struct MangaInfo {
    pub id: String,
    pub attributes: MangaAttributes,
}

#[derive(Debug, Deserialize)]
pub struct MangaAttributes {
    pub title: Title,
}

#[derive(Debug, Deserialize)]
pub enum Title {
    #[serde(rename = "en")]
    English(String),
    #[serde(rename = "ja-ro")]
    JapaneseRomanized(String),
}

#[derive(Debug, Default)]
pub struct ChapterSearchRequest<'a> {
    pub manga_id: &'a str,
    pub limit: i32,
    pub offset: i32,
}

impl<'a> ChapterSearchRequest<'a> {
    pub fn new(manga_id: &'a str) -> Self {
        Self {
            manga_id,
            ..Default::default()
        }
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = offset;
        self
    }

    pub async fn get(&self, client: &reqwest::Client) -> anyhow::Result<ChapterSearchResponse> {
        let url = format!("{BASE_URL}/manga/{id}/feed", id = self.manga_id);
        let mut params = HashMap::new();
        // Is there anything else that i can do?
        let limit = self.limit.to_string();
        let offset = self.offset.to_string();

        params.insert("limit", limit);
        params.insert("offset", offset);
        params.insert("includeExternalUrl", String::from("0"));
        params.insert("order[chapter]", String::from("asc"));
        params.insert("translatedLanguage[]", String::from("en"));
        let res = client.get(url).query(&params).send().await?;
        let feed_response = res.json().await?;
        Ok(feed_response)
    }
}

#[derive(Debug, Deserialize)]
pub struct ChapterSearchResponse {
    pub data: Vec<ChapterInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterInfo {
    pub id: String,
    #[serde(rename = "attributes")]
    pub attr: ChapterAttributes,
}

impl ChapterInfo {
    pub async fn download(&self, client: &reqwest::Client) -> anyhow::Result<()> {
        let url = format!("{BASE_URL}/at-home/server/{}", self.id);
        let download_path = format!("./chapter{}", self.attr.chapter);
        let res = client.get(url).send().await?;
        let info: DownloadInfo = res.json().await?;
        // TODO: command line args to save data
        info.save_at(Quality::High, download_path, &client).await
    }
}

#[derive(Debug, Deserialize)]
pub struct ChapterAttributes {
    pub volume: String,
    pub chapter: String,
    pub pages: u32,
}

#[derive(Debug, Deserialize)]
pub struct DownloadInfo {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: PageInfo,
}

pub enum Quality {
    High,
    Low,
}

impl DownloadInfo {
    pub async fn save_at<P>(
        &self,
        q: Quality,
        path: P,
        client: &reqwest::Client,
    ) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let data = match q {
            Quality::High => &self.chapter.data,
            Quality::Low => &self.chapter.data_saver,
        };

        let path = path.as_ref().display();
        let font = genpdf::fonts::from_files(
            "/usr/share/fonts/truetype/liberation/",
            "LiberationSans",
            Some(genpdf::fonts::Builtin::Helvetica),
        )?;
        let mut pdf = genpdf::Document::new(font);
        let pdf_path = format!("{path}.pdf");
        pdf.set_title(&pdf_path);
        let mut dec = genpdf::SimplePageDecorator::new();
        dec.set_margins(10);
        pdf.set_page_decorator(dec);
        for i in data {
            let url = format!("{}/data/{}/{}", self.base_url, self.chapter.hash, i);
            let res = client.get(url).send().await?;
            let bytes = res.bytes().await?;
            let seeker = Cursor::new(bytes.deref());
            let image =
                genpdf::elements::Image::from_reader(seeker)?.with_alignment(Alignment::Center);
            pdf.push(image);
        }
        pdf.render_to_file(&pdf_path)?;
        println!("INFO: Saved to {pdf_path}");
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
    hash: String,
    data: Vec<String>,
    #[serde(rename = "dataSaver")]
    data_saver: Vec<String>,
}
