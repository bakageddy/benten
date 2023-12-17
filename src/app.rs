use std::rc::Rc;

use ratatui::layout::{Layout, Constraint, Rect};

use crate::utils;

pub struct App {
    pub quit: bool,
    pub input: String,
    pub cursor: u16,
    pub client: reqwest::Client,
}

impl App {
    pub fn new() -> anyhow::Result<App> {
        let client = reqwest::ClientBuilder::new().user_agent(utils::USER_AGENT).build()?;
        Ok(App {
            input: String::new(),
            cursor: 0,
            client,
            quit: false,
        })
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn get_layout(frame_size: Rect) -> Rc<[Rect]>{
        let panes = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]).split(frame_size);
        panes
    }
}
