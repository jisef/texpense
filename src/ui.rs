use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType},
    Frame,
};
use ratatui::prelude::Span;
use ratatui::style::{Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Borders, Row, Tabs};
use crate::app::App;
use crate::tab::home::draw_home;
use crate::tab::statistics::draw_stats;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let size = frame.size();
    let title = Line::from(vec![
        Span::styled(format!(" {} ",app.tab_titles.get(0).unwrap()), if app.active_tab == 0 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        }else { 
            Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)
        }),
        Span::raw(" | "),
        Span::styled(format!(" {} ",app.tab_titles.get(1).unwrap()), if app.active_tab == 1 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)
        }),
    ]);

    /*let tabs = Tabs::new(app.tab_titles.clone())
        .select(app.active_tab)
        .highlight_style(Style::new().bold().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(title));*/
    let block = Block::default().borders(Borders::ALL).title(title).border_type(BorderType::Rounded);
    frame.render_widget(block, size);

    match app.active_tab {
        0 => draw_home(frame, size),
        1 => draw_stats(frame, size),
        _ => {}
    }

}
