use serde::Deserialize;

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

    // pub description: Description,
    // #[serde(alias = "lastVolume")]
    // last_vol: String,
    // #[serde(alias = "lastChapter")]
    // last_chapter: String,
    // status: Status,
    // #[serde(alias = "contentRating")]
    // content_rating: ContentRating,
    // #[serde(alias = "createdAt")]
    // created_at: String,
    // tags: Vec<Tag>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Title {
    pub en: Option<String>,
    pub ja: Option<String>,
    pub ja_ro: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    pub en: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum ContentRating {
    #[serde(alias = "safe")]
    Safe,

    #[serde(alias = "suggestive")]
    Suggestive,

    #[serde(alias = "erotica")]
    Erotica,

    #[serde(alias = "pornographic")]
    Pornographic,
}

#[derive(Deserialize, Debug)]
pub enum Status {
    #[serde(alias = "completed")]
    Completed,

    #[serde(alias = "ongoing")]
    Ongoing,

    #[serde(alias = "cancelled")]
    Cancelled,

    #[serde(alias = "hiatus")]
    Hiatus,
}

// pub struct Tag {
//     id: String,
//     attributes: TagAttributess,
// }
