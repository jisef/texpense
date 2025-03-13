use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders}
};
use ratatui::style::Stylize;
use ratatui::widgets::{Cell, Row, Table, Widget};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use sea_orm::{EntityTrait};

use time::{OffsetDateTime};
use account::Entity;
use tui_textarea::TextArea;
use crate::{db::get_db_connection, app::{App, TabBlock, TabManager}, entities::*, tab::home, entities};
use ratatui_input::Input;


pub const SELECTED_BLOCK: Style = Style::new().fg(Color::Cyan);

/// Enum to toggle through the blocks
pub struct Home {
    pub manager: TabManager<HomeBlock>,
    pub tf: bool,
    pub type_tf: char
}

impl Home {
    pub fn new() -> Home {
        Home {
            manager: TabManager::new(HomeBlock::Calendar),
            tf: false,
            type_tf: ' '
        }
    }
}
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
    let active_block = app.home.manager.current_block; // Get active block

    let default_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);


    let accounts = draw_accounts(active_block, default_block.clone());
    let payments = draw_payments(active_block, default_block.clone());
    let calendar = draw_calendar(active_block, default_block.clone());
    let templates = draw_templates(active_block, default_block.clone());
    let people = draw_people(active_block, default_block.clone());
    
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

    // POP-UP
    if app.home.tf {
        match active_block {
            HomeBlock::People => {

            },
            HomeBlock::Accounts => {
                match app.home.type_tf {
                    'a' => { // ADD

                        let centerd_popup = centered_rect(area.clone(), 35,10);
                        let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).bg(Color::DarkGray);
                        let mut text = TextArea::default();
                        text.set_block(block);


                        let input = Input::default();
                        
                        frame.render_widget(input, centerd_popup);
                        /*
                        let centerd_popup = centered_rect(area.clone(), 35,10);
                        let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
                        frame.render_widget(block.clone(), centerd_popup);
                        let mut tf_name  = TextArea::default();
                        tf_name.set_placeholder_text(" Name ");

                        let mut tf_description = TextArea::default();
                        tf_description.set_placeholder_text(" Description ");

                        let mut tf_amount = TextArea::default();
                        tf_amount.set_placeholder_text(" Amount ");
                        
                        let layout = Layout::default().constraints(vec![
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(60),
                        ]).direction(Direction::Horizontal).split(block.inner(area));

                        frame.render_widget(&tf_name, layout[0]);
                        frame.render_widget(&tf_amount, layout[1]);
                        frame.render_widget(&tf_description, layout[2]);*/
                    }
                    _ => {

                    }
                }
            },
            _ => {

            }
        }
    }

}

async fn draw_people<'a>(active_block: HomeBlock, default_block: Block<'a>) -> Block<'a> {
    let block = default_block.title(" People ").style( if active_block == HomeBlock::People {home::SELECTED_BLOCK } else {Style::default()});

    block
}

async fn draw_payments<'a>(active_block: HomeBlock, block: Block<'a>) -> Table<'a> {
    let db  = get_db_connection().await;
    let payments = payment::Entity::find().all(db);
    
    let block = block.title(" Payments ").style( if active_block == HomeBlock::Payments {home::SELECTED_BLOCK } else {Style::default()});
    let width = vec![
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(50)
    ];
    
    let payments = match payments.await {
        Ok(x) => x,
        Err(x) => panic!("Account not found {}", x),
    };
    let rows:Vec<Row> = payments.iter().map(|x| Row::new(vec![
        Cell::new(x.label.clone().to_string()),
        Cell::new(x.amount.clone().to_string()),
        Cell::new(x.description.clone().unwrap_or_default()),
    ])).collect();
    
    let table = Table::new(rows, width).header(Row::new(vec![
        Cell::from(" Label "),
        Cell::from(" Amount "),
        Cell::from(" Description ")
    ])).block(block);



    table
}

async fn draw_templates<'a>(active_block: HomeBlock, block: Block<'a>) -> Block<'a> {
    let db = get_db_connection();
    
    let b = block.title(" Templates ").style( if active_block == HomeBlock::Templates {home::SELECTED_BLOCK } else {Style::default()});
    let templates = match entities::template::Entity::find().all(db.await).await { 
        Ok(t) => t, 
        Err(x) => panic!("Error occured while retrieving templates: {}", x),
    };
    b
}

async fn draw_accounts<'a>(active_block: HomeBlock, block: Block<'a>) -> Table<'a> {
    // get all accounts
    let db = get_db_connection().await;
    let accounts = match Entity::find().all(db).await {
        Ok(x) => x,
        Err(x) => panic!("Account not found {}", x),
    };


    let rows: Vec<Row> = accounts.into_iter().map(|a| {
        Row::new(vec![
            Cell::from(a.name.clone().unwrap().to_string()),
            Cell::from(a.amount.clone().unwrap().to_string()),
            Cell::from(if a.description.is_some() { a.description.clone().unwrap() } else { String::new() }),
        ])
    }).collect();


    let width_table = [
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Length(40),
    ];

    let block = block.title(" Accounts ").style( if active_block == HomeBlock::Accounts {home::SELECTED_BLOCK } else {Style::default()});

    let table = Table::new(rows, width_table)
        .block(block)
        .header(Row::new(vec![
            Cell::from("Name"),
            Cell::from("Amount"),
            Cell::from("Description"),
        ]).style(Style::new().bold()));


    table
}

/// Shows the data red when the daily budget is overuse else green
async fn draw_calendar<'a>(active_block: HomeBlock, block: Block<'a>) -> Monthly<'a, CalendarEventStore> {

    //todo!("Make it beatiful")
    let block = block.title(" Calendar ").style( if active_block == HomeBlock::Calendar {home::SELECTED_BLOCK } else {Style::default()});


    let mut start = OffsetDateTime::now_local().unwrap().date();

    let cal: Monthly<CalendarEventStore> = Monthly::new(
        OffsetDateTime::now_local().unwrap().date(),
        CalendarEventStore::default()
    ).show_surrounding(Style::default())
        .show_month_header(Style::default())
        .show_weekdays_header(Style::default()).block(block);

    cal
}


fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_add_account_block<'a>() -> Block<'a> {
    let mut tf_name  = TextArea::default();
    tf_name.set_placeholder_text(" Name ");

    let mut tf_description = TextArea::default();
    tf_description.set_placeholder_text(" Description ");

    let mut tf_amount = TextArea::default();
    tf_amount.set_placeholder_text(" Amount ");

    let layout = Layout::default().constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ]).direction(Direction::Horizontal);

    let block = Block::default().title("SIGMA").borders(Borders::ALL).border_type(BorderType::Thick).border_type(BorderType::Rounded);

    block
}
