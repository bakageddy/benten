use std::{collections::HashMap, fs, path::Path, rc::Rc, usize};

use ratatui::layout::{Constraint, Layout, Rect};

use crate::{
    types::*,
    utils::{self, API_BASE_URL},
};

pub struct App {
    pub quit: bool,
    pub input: String,
    pub search_result: Option<MangaSearchResult>,
    pub download_result: Option<MangaChapter>,
    pub cursor: u16,
    pub client: reqwest::blocking::Client,
}

impl App {
    pub fn new() -> anyhow::Result<App> {
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(utils::USER_AGENT)
            .build()?;
        Ok(App {
            input: String::new(),
            search_result: None,
            download_result: None,
            cursor: 0,
            client,
            quit: false,
        })
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn get_layout(frame_size: Rect) -> Rc<[Rect]> {
        let panes = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame_size);
        panes
    }

    pub fn send_signal(&mut self, signal: Signal) {
        match signal {
            Signal::Search => {
                self.search_result = self.get_manga_list();
            },
            Signal::Select(_x) => {
                todo!();
            }
        }
    }

    pub fn get_manga_list(&mut self) -> Option<MangaSearchResult> {
        let url = format!("{base}/manga", base = utils::API_BASE_URL);
        let mut params = HashMap::new();
        params.insert("title", self.input.as_str());
        params.insert("limit", "10");

        let res = self.client.get(url).query(&params).send();
        match res {
            Ok(res) => {
                let json = res.json::<MangaSearchResult>();
                json.ok()
            }
            Err(_) => {
                println!("Error sending request!");
                None
            }
        }
    }

    pub fn get_feed(&self, manga: &Manga) -> Option<MangaChapterResult> {
        let url = format!("{}/manga/{}/feed", API_BASE_URL, manga.id);
        let mut params = HashMap::new();
        params.insert("translatedLanguage[]", "en");
        params.insert("includeExternalUrl", "0");
        params.insert("order[chapter]", "asc");
        let res = self.client.get(url).query(&params).send();
        match res {
            Ok(res) => return res.json::<MangaChapterResult>().ok(),
            Err(e) => {
                eprintln!("What happened!?: {e}");
                return None;
            }
        }
    }

    pub fn download_chapter(&self, chapter: &MangaChapter) -> Result<(), ()> {
        let url = format!("{}/at-home/server/{}", API_BASE_URL, chapter.id);
        if let Ok(res) = self.client.get(url).send() {
            if let Ok(metadata) = res.json::<DownloadChapter>() {
                let dir = Path::new(&chapter.attributes.title);
                if !Path::exists(dir) {
                    fs::create_dir(dir).expect("Should not panic!");
                }
                let mut i = 1;
                for page in metadata.chapter.data {
                    let page_download_url = format!(
                        "{}/data/{}/{}",
                        metadata.base_url, metadata.chapter.hash, page
                    );
                    if let Ok(page_download_res) = self.client.get(page_download_url).send() {
                        let _ = fs::write(
                            // pad with three zeros before
                            format!("./{dir}/{i:0>3}.png", dir=dir.display()),
                            page_download_res.bytes().unwrap(),
                        );
                        i += 1;
                    } else {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
        Ok(())
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor as usize == self.input.len() {
            return;
        }
        self.cursor += 1;
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor as usize, c);
        self.cursor += 1;
    }

    pub fn delete_current_char(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.input.remove((self.cursor - 1) as usize);
        self.cursor -= 1;
    }
}
