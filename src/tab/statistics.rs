use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BarChart, Block, Borders, Chart, Paragraph};
use ratatui::widgets::canvas::Rectangle;
use crate::app::{TabBlock, TabManager};
use crate::tab::home::HomeBlock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticsBlock {
    Overview,
    Trends,
    History,
}

impl TabBlock for StatisticsBlock {
    fn next(&self) -> Self {
        match self {
            StatisticsBlock::Overview => StatisticsBlock::Trends,
            StatisticsBlock::Trends => StatisticsBlock::History,
            StatisticsBlock::History => StatisticsBlock::Overview, // Wrap around
        }
    }

    fn previous(&self) -> Self {
        match self {
            StatisticsBlock::Overview => StatisticsBlock::History, // Wrap around
            StatisticsBlock::Trends => StatisticsBlock::Overview,
            StatisticsBlock::History => StatisticsBlock::Trends,
        }
    }

    fn key_bindings(&self) -> &'static str {
        todo!()
    }
}

pub fn handle_statistics_key_event(manager: &mut TabManager<StatisticsBlock>, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => manager.next_block(),
        KeyCode::BackTab => manager.previous_block(),

        // Example bindings for statistics tab
        KeyCode::Char('s') if manager.current_block == StatisticsBlock::Overview => {
            println!("Showing statistics overview...");
        }

        _ => {}
    }
}

pub fn draw_stats(fram: &mut Frame, area: Rect) {
    let layout = Layout::default().direction(Direction::Horizontal);
    /*let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .bar_width(3)
        .bar_gap(2)
        .bar_style(Style::new().yellow().on_red())
        .value_style(Style::new().red().bold())
        .data(&[("B0", 3), ("B1", 3), ("B2", 3), ("B3", 3), ("B4", 4)]);
    let block = Block::new().title(" Stats ").borders(Borders::ALL);


    fram.render_widget(chart, area);

        fram.render_widget(block, area);*/
}