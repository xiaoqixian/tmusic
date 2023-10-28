// Date: Chu Oct 26 10:25:47 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::layout::{Rect, Constraint};
use tui::widgets::{
    Widget, 
    Block as TuiBlock, 
    Borders, 
    BorderType
};
use tui::style::{Style, Color};

use super::component::{CompMode, Component, CompState};

pub struct Block<C> {
    inner: C,
    area: Rect,
    title: Option<String>,
    mode: CompMode
}

impl<C> Block<C> {
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            title: Default::default(),
            area: Default::default(),
            mode: CompMode::Leave
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    fn border_type(&self) -> BorderType {
        match self.mode {
            CompMode::Enter |
            CompMode::Leave => BorderType::Rounded,

            CompMode::Hover => BorderType::Double
        }
    }

    fn border_style(&self) -> Style {
        match self.mode {
            CompMode::Enter |
            CompMode::Hover => Style::default().fg(Color::Blue),

            CompMode::Leave => Style::default().fg(Color::White)
        }
    }
}

impl<C> Component for Block<C>
where C: Component
{
    fn set_area(&mut self, area: Rect) {
        self.area = area;
        self.inner
            .set_area(
                TuiBlock::default()
                .borders(Borders::ALL)
                .inner(area)
            );
    }

    fn get_constraint(&self) -> Constraint {
        match self.inner.get_constraint() {
            Constraint::Max(max) => Constraint::Max(max+2),
            Constraint::Min(min) => Constraint::Min(min+2),
            Constraint::Ratio(a, b) => Constraint::Ratio(a, b),
            Constraint::Length(len) => Constraint::Length(len+2),
            Constraint::Percentage(per) => Constraint::Percentage(per)
        }
    }

    #[inline]
    fn read_event(&mut self, event: crossterm::event::Event) -> CompState {
        self.inner.read_event(event)
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let mut block = TuiBlock::default()
            .borders(Borders::ALL)
            .border_style(self.border_style())
            .border_type(self.border_type())
            .title(
                self.title.as_ref()
                .map(|title| title.as_str())
                .unwrap_or("")
            );

        block.render(self.area, buffer);
        self.inner.render(buffer);
    }

    fn alter_mode(&mut self, mode: CompMode) -> CompState {
        let comp_state = self.inner.alter_mode(mode);
        match comp_state {
            CompState::Exit => {},
            _ => self.mode = mode
        }
        comp_state
    }

    #[inline]
    fn update_duration(&self) -> Option<std::time::Duration> {
        self.inner.update_duration()
    }
}
