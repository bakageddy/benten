use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

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
        let mut titles = String::new();

        // What the fuck is this?
        // TODO: refactor this reference-ful code!
        match app.results {
            Some(ref res) => {
                for i in &res.data {
                    if let Some(ref title) = i.attributes.title.en {
                        titles.push_str(title);
                        titles.push('\n');
                    }
                }
            },
            None => {titles = "No results".to_string();}
        };


        let hello = Paragraph::new(titles).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("benten"),
        );
        frame.set_cursor(app.cursor + chunks[0].x + 1, chunks[0].y + 1);
        frame.render_widget(input_area, chunks[0]);
        frame.render_widget(hello, chunks[1]);
    });
}
