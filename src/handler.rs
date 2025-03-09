use crate::app::{App, AppResult, TabManager};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Tab => {
            match app.active_tab {
                0 => app.home_manager.next_block(),      // Home tab
                1 => app.statistics_manager.next_block(), // Statistics tab
                _ => {}
            }
        }
        KeyCode::BackTab => {
            match app.active_tab {
                0 => app.home_manager.previous_block(),
                1 => app.statistics_manager.previous_block(),
                _ => {}
            }
        }
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Char('1') => {
            app.active_tab = 0;
        }
        KeyCode::Char('2') => {
            app.active_tab = 1;
        }
        KeyCode::Char('3') => {
            app.active_tab = 2;
        }
        _ => {}
    }
    Ok(())
}
