use ratatui::Frame;
use ratatui::layout::{Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{BarChart, Block, Borders, Chart, Paragraph};
use ratatui::widgets::canvas::Rectangle;

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