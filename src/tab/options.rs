use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::{App, TabBlock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsBlock {
    WeekGoals
}
impl TabBlock for OptionsBlock {
    fn next(&self) -> Self {
        match self {
            _ => Self::WeekGoals
        }
    }

    fn previous(&self) -> Self {
        match self {
            _ => Self::WeekGoals
        }
    }

    fn key_bindings(&self) -> &'static str {
        match self {
            OptionsBlock::WeekGoals => "",
        }
    }
}




pub async fn draw_options(frame: &mut Frame<'_>, area: Rect, app: &App) { }
