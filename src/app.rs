use std::error;
use crossterm::event::KeyEvent;
use crate::tab::home::{ Home};
use crate::tab::statistics::{handle_statistics_key_event, StatisticsBlock};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.

pub struct App {
    pub running: bool,
    pub active_tab: usize, // 0 = first tab, 1 = second tab, etc.
    pub tab_titles: Vec<String>,
    pub statistics_manager: TabManager<StatisticsBlock>,
    pub home: Home,
}

impl App {
    pub fn tick(&self) {}
    pub fn new() -> Self {
        let titles = vec![
            "Home".to_string(),
            "Statistics".to_string(),
        ];
        
        Self {
            running: true,
            active_tab: 0,
            tab_titles: titles.clone(),
            statistics_manager: TabManager::new(StatisticsBlock::Overview),
            home: Home::new() 
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % self.tab_titles.len();
    }

    pub fn previous_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = self.tab_titles.len() - 1;
        } else {
            self.active_tab -= 1;
        }
    }
}



pub trait TabBlock {
    fn next(&self) -> Self;
    fn previous(&self) -> Self;
    fn key_bindings(&self) -> &'static str;
}

pub struct TabManager<T: TabBlock> {
    pub current_block: T,
}

impl<T: TabBlock> TabManager<T> {
    pub fn new(start_block: T) -> Self {
        Self { current_block: start_block }
    }

    pub fn next_block(&mut self) {
        self.current_block = self.current_block.next();
    }

    pub fn previous_block(&mut self) {
        self.current_block = self.current_block.previous();
    }
}

