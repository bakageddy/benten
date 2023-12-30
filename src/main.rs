use utils::{shutdown, start};

mod app;
mod events;
mod types;
mod ui;
mod utils;

fn main() -> anyhow::Result<()> {
    let mut terminal = start()?;
    let mut app = app::App::new()?;
    loop {
        ui::draw_frame(&mut terminal, &app);
        let _ = events::handle_events(&mut app);
        if app.quit {
            break;
        }
    }
    let _ = shutdown()?;
    Ok(())
}
