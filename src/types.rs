use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MangaSearchResult {
    result: SearchResult,
    response: String,
    data: Vec<Manga>,
}

#[derive(Deserialize, Debug)]
pub enum SearchResult {
    Ok,
    Error,
}

#[derive(Deserialize, Debug)]
pub struct Manga {
    id: String,
    manga_type: String,
    attributes: Attribute,
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    year: i32,
    title: Title,
    alt_title: Title,
    desc: Description,
    last_vol: String,
    last_chapter: String,
    status: Status,
    content_rating: ContentRating,
    created: String,
    last_updated: String,
    // tags: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
pub struct Title {
    en: Option<String>,
    ja: Option<String>,
    ja_ro: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    en: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Deserialize, Debug)]
pub enum Status {
    Completed,
    Ongoing,
    Cancelled,
    Hiatus,
}

// pub struct Tag {
//     id: String,
//     attributes: TagAttributess,
// }
