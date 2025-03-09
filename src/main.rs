use std::io;
use dotenvy::dotenv;
use ratatui::{backend::CrosstermBackend, Terminal};
use sea_orm::{EntityTrait};
use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;
pub mod entities;
pub mod tab;
mod db;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let _ = db::initialize_db_connection().await?;
   
    
    
    let db = db::get_db_connection().await;
    
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    
    tui.exit()?;
    Ok(())
}
