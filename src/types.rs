pub struct Manga {
    id: String,
    manga_type: String,
    attributes: Attribute,
}

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

pub struct Title {
    en: Option<String>,
    ja: Option<String>,
    ja_ro: Option<String>,
}

pub struct Description {
    en: Option<String>,
}

pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

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
