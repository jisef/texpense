use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem}
};
use sea_orm::{EntityTrait};
use account::Entity;
use crate::{db::get_db_connection, app::{App, TabBlock, TabManager}, entities::*, tab::home, entities};

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

    fn key_bindings(&self) -> &'static str {
        match self { 
            HomeBlock::People => " [a] Add | [e] Edit",
            HomeBlock::Payments => " [b] Remove | [c] Swap",
            HomeBlock::Calendar => "",
            HomeBlock::Accounts => "",
            HomeBlock::Templates => "",
        }
    }
}

pub async fn draw_home(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let active_block = app.home_manager.current_block; // Get active block

    let default_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    /// .style( if active_block == HomeBlock::Accounts {home::selected} else {Style::default()});
    let accounts = draw_accounts(active_block, default_block.clone());
    let payments = draw_payments(active_block, default_block.clone());
    let calendar = draw_calendar(active_block, default_block.clone());
    let templates = draw_templates(active_block, default_block.clone());
    let people = draw_peopple(active_block, default_block.clone());
    
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70)])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(32),
            Constraint::Percentage(32),
            Constraint::Percentage(36)
        ])
        .split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(main_layout[1]);


    // RIGHT
    frame.render_widget(templates.await, right_layout[0]);
    frame.render_widget(payments.await, right_layout[1]);

    // LEFT
    frame.render_widget(calendar.await, left_layout[0]);
    frame.render_widget(accounts.await, left_layout[1]);
    frame.render_widget(people.await, left_layout[2]);

}

async fn draw_peopple<'a>(active_block: HomeBlock, default_block: Block<'a>) -> Block<'a> {
    let block = default_block.title(" People ").style( if active_block == HomeBlock::People {home::selected} else {Style::default()});
    block
}

async fn draw_payments<'a>(active_block: HomeBlock, block: Block<'a>) -> Block<'a> {
    let block = block.title(" Payments ").style( if active_block == HomeBlock::Payments {home::selected} else {Style::default()});
    
    
    
    block
}

async fn draw_templates<'a>(active_block: HomeBlock, block: Block<'a>) -> Block<'a> {
    
    
    let db = get_db_connection();
    
    let b = block.title(" Templates ").style( if active_block == HomeBlock::Templates {home::selected} else {Style::default()});
    let templates = match entities::template::Entity::find().all(db.await).await { 
        Ok(t) => t, 
        Err(x) => panic!("Error occured while retrieving templates: {}", x),
    };
    b
}

async fn draw_accounts<'a>(active_block: HomeBlock, block: Block<'a>) -> List<'a> {
    // get all accounts
    let db = get_db_connection().await;
    let accounts = match Entity::find().all(db).await {
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

    let block = block.title(" Accounts ").style( if active_block == HomeBlock::Accounts {home::selected} else {Style::default()});
   
    
    let list = List::new(list_items).block(block);




    


    list
}

pub fn handle_home_key_event(manager: &mut TabManager<HomeBlock>, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => manager.next_block(),
        KeyCode::BackTab => manager.previous_block(),

        // Custom key bindings per block
        KeyCode::Char('p') if manager.current_block == HomeBlock::People => {
            println!("Adding a person...");
        }
        KeyCode::Char('d') if manager.current_block == HomeBlock::People => {
            println!("Deleting a person...");
        }
        KeyCode::Char('n') if manager.current_block == HomeBlock::Payments => {
            println!("Creating a new payment...");
        }
        KeyCode::Char('v') if manager.current_block == HomeBlock::Payments => {
            println!("Viewing payment details...");
        }
        KeyCode::Char('a') if manager.current_block == HomeBlock::Calendar => {
            println!("Adding an event to the calendar...");
        }
        KeyCode::Char('e') if manager.current_block == HomeBlock::Calendar => {
            println!("Editing calendar event...");
        }
        KeyCode::Char('t') if manager.current_block == HomeBlock::Templates => {
            println!("Loading a template...");
        }
        KeyCode::Char('s') if manager.current_block == HomeBlock::Templates => {
            println!("Saving a template...");
        }
        KeyCode::Char('r') if manager.current_block == HomeBlock::Accounts => {
            println!("Refreshing accounts...");
        }
        KeyCode::Char('m') if manager.current_block == HomeBlock::Accounts => {
            println!("Managing accounts...");
        }

        _ => {}
    }
}

async fn draw_calendar<'a>(active_block: HomeBlock, block: Block<'a>) -> Block<'a> {
    let block = block.title(" Calendar ").style( if active_block == HomeBlock::Calendar {home::selected} else {Style::default()});
    
    block
}
