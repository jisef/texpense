use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use sea_orm::{EntityTrait};
use crate::app::{App, TabBlock};
use crate::db;
use crate::entities::account;
use crate::tab::home;

pub const selected: Style = Style::new().fg(Color::Cyan);

/// Enum to toggle though the blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeBlock {
    People,
    Payments,
    Templates,
    Calendar,
    Accounts,
}


impl TabBlock for HomeBlock {
    fn next(&self) -> Self {
        match self {
            HomeBlock::People => HomeBlock::Payments,
            HomeBlock::Payments => HomeBlock::Calendar,
            HomeBlock::Calendar => HomeBlock::Templates,
            HomeBlock::Templates => HomeBlock::Accounts,
            HomeBlock::Accounts => HomeBlock::People, 
        }
    }

    fn previous(&self) -> Self {
        match self {
            HomeBlock::People => HomeBlock::Accounts, 
            HomeBlock::Payments => HomeBlock::People,
            HomeBlock::Calendar => HomeBlock::Payments,
            HomeBlock::Templates => HomeBlock::Calendar,
            HomeBlock::Accounts => HomeBlock::Templates,
        }
    }
}





/*pub async fn draw_home(frame: &mut Frame<'_>, area: Rect, active_block:HomeBlock) {
    let accounts = draw_accounts(active_block).await;

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
    frame.render_widget(accounts,left_layout[1] );


}*/
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

async fn draw_accounts<'a>(active_block: HomeBlock) -> List<'a> {
    // get all accounts
    let db =  match db::establish_connection().await {
        Ok(db) => db,
        Err(x) => panic!("Connection couldnt be established {}", x),
    };
    let accounts = match account::Entity::find().all(&db).await {
        Ok(x) => x,
        Err(x) => panic!("Account not found {}", x),
    };

    // Convert accounts into list items todo!("noch ändern und verschönern")
    let list_items: Vec<ListItem> = accounts
        .iter()
        .map(|acc| {
            let name = acc.name.clone().unwrap_or_else(|| "Unnamed".to_string());
            let amount = acc.amount.map_or("0.00".to_string(), |amt| amt.to_string());
            let description = acc.description.clone().unwrap_or_else(|| "No description".to_string());

            ListItem::new(format!(" {},  {},  {}", name, amount, description))
        })
        .collect();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Accounts ").style( if active_block == HomeBlock::Accounts {home::selected} else {Style::default()});

    let list = List::new(list_items).block(block);




    //todo!("TAB für durchwechseln geht noch nicht");


    list
}

pub async fn draw_home(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let active_block = app.home_manager.current_block; // Get active block
    let accounts = draw_accounts(active_block).await;

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(main_layout[0]);

    let expenses_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Monthly expenses ")
        .style(if active_block == HomeBlock::People { home::selected } else { Style::default() });

    let payments_block = draw_payments()
        .style(if active_block == HomeBlock::Payments { home::selected } else { Style::default() });

    frame.render_widget(expenses_block, left_layout[0]);
    frame.render_widget(payments_block, left_layout[1]);
    frame.render_widget(accounts, left_layout[2]);
}
