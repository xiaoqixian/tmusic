// Date: Wed Oct 25 15:02:07 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    layout::{Constraint, Direction, Rect},
    widgets::{Widget, Gauge, Block, BorderType, Borders},
    buffer::Buffer, style::{Style, Color}
};

use super::component::{
    CompState, 
    Component, 
    CompMode,
    Query,
    QueryResponse,
    Attribution
};

pub struct ProgressBar {
    constraint: Constraint,
    title: Option<String>,
    area: Option<Rect>,
    comp_mode: CompMode
}

impl ProgressBar {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c,
            title: None,
            area: None,
            comp_mode: CompMode::Leave
        }
    }
}

impl Component for ProgressBar {
    fn query(&self, q: Query) -> QueryResponse {
        match q {
            Query::Title => QueryResponse::Title(None),
            Query::Constraint => 
                QueryResponse::Constraint(self.constraint.clone()),
            Query::UpdateDuration => 
                QueryResponse::UpdateDuration(Some(std::time::Duration::from_secs(1)))
        }
    }

    fn set_attr(&mut self, attr: Attribution) -> Option<CompState> {
        match attr {
            Attribution::Area(area) => {
                self.area = Some(area);
                None
            }
            Attribution::Mode(mode) => self.alter_mode(mode)
        }
    }

    /// Currently, ProgressBar don't accept any event
    /// its not even enterable.
    #[inline]
    fn feed_event(&mut self, _: crossterm::event::Event) -> CompState {
        CompState::Exit
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let area = match self.area {
            None => return,
            Some(area) => area
        };

        Gauge::default()
            .style(Style::default().fg(Color::Yellow))
            .percent(50)
            .render(area, buffer);
    }

    #[inline]
    fn alter_mode(&mut self, mode: CompMode) -> Option<CompState> {
        match mode {
            CompMode::Enter => Some(CompState::Exit),
            m => {
                self.comp_mode = m;
                None
            }
        }
    }
}
