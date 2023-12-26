use std::{collections::HashMap, rc::Rc, usize};

use ratatui::layout::{Constraint, Layout, Rect};

use crate::{types, utils};

pub struct App {
    pub quit: bool,
    pub input: String,
    pub results: Option<types::MangaSearchResult>,
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
            results: None,
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

    pub fn get_manga_list(&mut self) {
        self.results = None;
        let url = format!("{base}/manga", base = utils::API_BASE_URL);
        let mut params = HashMap::new();
        params.insert("title", self.input.as_str());
        params.insert("limit", "1");

        let res = self.client.get(url)
            .query(&params).send();
        match res {
            Ok(res) => {
                let json = res.json::<types::MangaSearchResult>();
                self.results = json.ok();
            }
            Err(_) => {
                self.results = None;
                println!("Error sending request!");
            }
        }
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
