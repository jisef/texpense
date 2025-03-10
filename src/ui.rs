use crate::app::App;
use crate::tab::home::{draw_home, HomeBlock};
use crate::tab::statistics::draw_stats;
use ratatui::layout::Rect;
use ratatui::prelude::Span;
use ratatui::style::{Modifier, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Borders;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType},
    Frame,
};

/// Renders the user interface widgets.
pub async fn render(app: &mut App, frame: &mut Frame<'_>) {
    let size = frame.size();

    // Define the title with tab styles
    let title = Line::from(vec![
        Span::styled(
            format!(" {} ", app.tab_titles.get(0).unwrap()),
            if app.active_tab == 0 {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)
            },
        ),
        Span::raw(" | "),
        Span::styled(
            format!(" {} ", app.tab_titles.get(1).unwrap()),
            if app.active_tab == 1 {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::ITALIC)
            },
        ),
    ]);

    // Create the main block with borders
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Rounded);

    // Render the outer block
    frame.render_widget(outer_block, size);

    // Compute the inner area (exclude 1px border on all sides)
    let inner_area = Rect {
        x: size.x + 2,               // Move right todo!("fix borders )
        y: size.y + 1,               // Move down
        width: size.width.saturating_sub(2),  // Shrink width
        height: size.height.saturating_sub(2), // Shrink height
    };

    // Render content **inside** the block
    match app.active_tab {
        0 => draw_home(frame, inner_area, &app).await, // Use the inner area
        1 => draw_stats(frame, inner_area), 
        _ => {}
    }
}