use serde::Deserialize;

pub enum Signal {
    Search,
    Select(Manga),
}

#[derive(Deserialize, Debug)]
pub struct MangaSearchResult {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga>,
}

#[derive(Deserialize, Debug)]
pub struct Manga {
    pub id: String,
    #[serde(alias = "type")]
    pub manga_type: String,
    pub attributes: Attribute,
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub year: i32,
    pub title: Title,
}

#[derive(Deserialize, Debug, Default)]
pub struct Title {
    pub en: Option<String>,
    pub ja: Option<String>,
    pub ja_ro: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MangaChapterResult {
    pub result: String,
    pub response: String,
    pub data: Vec<MangaChapter>,
}

#[derive(Deserialize, Debug)]
pub struct MangaChapter {
    pub id: String,
    #[serde(alias = "type")]
    pub chap: String,
    pub attributes: ChapterAttributes,
}

#[derive(Deserialize, Debug)]
pub struct ChapterAttributes {
    pub volume: String,
    pub chapter: String,
    pub title: String,
    pub pages: i32,
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct DownloadChapter {
    pub result: String,
    #[serde(alias = "baseUrl")]
    pub base_url: String,
    pub chapter: ChapterMetaData,
}

#[derive(Deserialize, Debug)]
pub struct ChapterMetaData {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(alias = "dataSaver")]
    pub data_saver: Vec<String>,
}
