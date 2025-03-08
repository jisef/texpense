
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;

use ratatui::widgets::{Block, BorderType, Borders};
use sea_orm::prelude::Decimal;
use sea_orm::{NotSet, Set};
use account::ActiveModel;
use crate::entities::account;
use crate::entities::prelude::Account;

pub fn draw_home(frame: &mut Frame, area: Rect) {
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
fn draw_payments<'a>() -> Block<'a> {
       Block::default()
           .borders(Borders::ALL)
           .border_type(BorderType::Rounded)
           .title(" Payments ")
}

fn draw_templates<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Templates ")
    
}

fn draw_accounts() {

    let new_account = account::ActiveModel {
        name: Set(Some("Savings Account".to_owned())),
        amount: Set(Some(Decimal::new(1000, 2))), // 10.00
        description: Set(Some("My savings account".to_owned())),
        id_account: NotSet, // This field will be auto-incremented
    };
}