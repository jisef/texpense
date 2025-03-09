use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState};
use sea_orm::prelude::Decimal;
use sea_orm::{DbConn, EntityTrait, NotSet, Set};
use crate::db;
use crate::entities::account;

pub async fn draw_home(frame: &mut Frame<'_>, area: Rect) {
    
    let accounts = draw_accounts().await;
    
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

async fn draw_accounts<'a>() -> List<'a> {
    // get all accounts
    let db =  match db::establish_connection().await {
        Ok(db) => db,
        Err(x) => panic!("Connection couldnt be established {}", x),
    };
    let accounts = match account::Entity::find().all(&db).await {
        Ok(x) => x,
        Err(x) => panic!("Account not found {}", x),
    };

    // Convert accounts into list items
    let list_items: Vec<ListItem> = accounts
        .iter()
        .map(|acc| {
            let name = acc.name.clone().unwrap_or_else(|| "Unnamed".to_string());
            let amount = acc.amount.map_or("0.00".to_string(), |amt| amt.to_string());
            let description = acc.description.clone().unwrap_or_else(|| "No description".to_string());

            ListItem::new(format!(" {},  {},  {}", name, amount, description))
        })
        .collect();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Accounts");

    let list = List::new(list_items).block(block);

    




    
    list
}