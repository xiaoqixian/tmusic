// Date: Wed Oct 25 15:02:07 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    layout::{Constraint, Direction, Rect},
    widgets::{Widget, Gauge, Block, BorderType, Borders},
    buffer::Buffer
};

use super::component::{
    Component, 
    CompMode, 
    CompState,
    StatefulComponent,
    DefaultFrameStyle
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
    #[inline]
    fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    /// Currently, ProgressBar don't accept any event
    /// its not even enterable.
    #[inline]
    fn read_event(&mut self, _: crossterm::event::Event) -> CompState {
        CompState::Exit
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let area = match self.area {
            None => return,
            Some(area) => area
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(self.border_type())
            .border_style(self.border_style())
            .title(
                self.title
                    .as_ref()
                    .map(|t| t.as_str())
                    .unwrap_or("")
            );

        let gauge = Gauge::default()
            .block(block)
            .percent(50);

        gauge.render(area, buffer);
    }

    #[inline]
    fn alter_mode(&mut self, mode: CompMode) -> CompState {
        match mode {
            CompMode::Enter => CompState::Exit,
            m => {
                self.comp_mode = m;
                CompState::Stay
            }
        }
    }

    #[inline]
    fn update_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(1))
    }
}

impl StatefulComponent for ProgressBar {
    fn comp_mode(&self) -> CompMode {
        self.comp_mode
    }
}
