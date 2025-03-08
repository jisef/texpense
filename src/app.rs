use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub active_tab: usize,
    pub tab_titles: Vec<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            active_tab: 0,
            tab_titles: vec!["Home", "Statistics"]
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
    
    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % self.tab_titles.len();
    }
    
    pub fn previous_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = self.tab_titles.len() - 1;
        } else if self.active_tab != 0 {
            self.active_tab -= 1;   
        } else if self.active_tab == self.tab_titles.len() - 1 { 
            self.active_tab = 0;
        }
    }
}
