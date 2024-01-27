use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};

use crate::{app::App, utils};
pub fn draw_frame(term: &mut utils::Term, app: &App) {
    let _ = term.draw(|frame| {
        let chunks = App::get_layout(frame.size());
        let input_area = Paragraph::new(app.input.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Manga Search"),
        );
        // let mut titles = String::new();

        // What the fuck is this?
        // TODO: refactor this reference-ful code!
        // match app.results {
        //     Some(ref res) => {
        //         let manga = &res.data[0];
        //         if let Some(chaps) = app.get_feed(manga) {
        //             app.download_chapter(&chaps.data[0]);
        //         } else {
        //             todo!();
        //         }
        //     },
        //     None => {titles = "No results".to_string();}
        // };

        let search_list: Vec<ListItem>;
        match app.search_result {
            Some(ref res) => {
                search_list = res
                    .data
                    .iter()
                    .map(|m| {
                        ListItem::new(
                            match m.attributes.title.en {
                                Some(ref t) => {t.clone()},
                                None => {String::from("No English Title!")}
                            }
                        )
                    })
                    .collect();
            },
            None => {
                search_list = vec![ListItem::new("No Search Result!")];
            }
        }

        let list = List::new(search_list).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Search Result"),
        );
        frame.set_cursor(app.cursor + chunks[0].x + 1, chunks[0].y + 1);
        frame.render_widget(input_area, chunks[0]);
        frame.render_widget(list, chunks[1]);
    });
}
