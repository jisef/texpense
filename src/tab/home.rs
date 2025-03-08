use std::fmt::format;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, BorderType, Borders};
//use ratatui::symbols::Marker::Block;

pub fn draw_home(frame: &mut Frame, area: Rect) {
    let payments = draw_payments();
    let templates = draw_templates();
    let accounts = draw_accounts();

    let main_layout = Layout::default().direction(Direction::Horizontal).constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]).split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![ Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]).split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(35),
        ]).split(main_layout[1]);
    
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Monthly expenses ");
    frame.render_widget(block, left_layout[0]);

    

   
}


fn draw_payments() {
    
}

fn draw_templates() {

}

fn draw_accounts() {

}