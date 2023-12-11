use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen, disable_raw_mode}};
use serde::{Deserialize};
use std::{fs::{File, self}, io::stdout};

use reqwest::ClientBuilder;
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

#[derive(Debug, Deserialize)]
enum ContentRating {
    safe,
    suggestive,
    erotica,
    pornographic
}

#[derive(Debug, Deserialize)]
enum Title {
    en(String),
    ja(String),
    ja_ro(String),
}

#[derive(Debug, Deserialize)]
struct Description {
    en: Option<String>,
    ru: Option<String>,
    ja: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    title: Title,
    description: Description,
}
#[derive(Debug, Deserialize)]
struct Manga {
    id: String,
    attributes: Attributes,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    data: Vec<Manga>,
    total: i64,
}


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
    
    let json = res.json::<SearchResponse>().await?;
    let mut titles = String::new();
    for manga in json.data {
        match manga.attributes.title {
            Title::en(t) => titles.push_str(&t),
            _ => continue,
        }
    }
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut count = 0;
    while true {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new(titles.to_string()), Rect::new(10, 10, 10, 10));
        })?;
        count += 1;
        if count == 1000000 {
            break;
        }
    }
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
