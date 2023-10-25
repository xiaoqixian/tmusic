// Date: Wed Oct 25 21:05:42 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::rc::Rc;

use tui::{
    widgets::BorderType,
    layout::Constraint,
    buffer::Buffer,
    style::{Style, Color}
};

use crossterm::event::Event;

use super::component::{CompMode, Component};

enum Mode {
    Entered,
    Hover,
    Left
}

pub struct Enterable<C> {
    inner: C,
    cursor_mode: Mode
}

impl<C> Enterable<C> {
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            cursor_mode: Mode::Left
        }
    }
}

impl<C> Component for Enterable<C>
where C: Component
{
    #[inline]
    fn set_area(&mut self, area: tui::layout::Rect) {
        self.inner.set_area(area)
    }

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.inner.get_constraint()
    }

    fn read_event(&mut self, event: Event) -> CompMode {
        CompMode::Stay
    }

    #[inline]
    fn render(&self, buffer: &mut Buffer) {
        self.inner.render(buffer);
    }

    #[inline]
    fn is_enterable(&self) -> bool {
        true
    }

    #[inline]
    fn enter(&mut self) {
        self.cursor_mode = Mode::Entered;
        self.inner.enter();
    }

    #[inline]
    fn hover(&mut self) {
        self.cursor_mode = Mode::Hover;
        self.inner.hover();
    }

    #[inline]
    fn leave(&mut self) {
        self.cursor_mode = Mode::Left;
        self.inner.leave();
    }

    #[inline]
    fn border_type(&self) -> Option<BorderType> {
        Some(match self.cursor_mode {
            Mode::Entered |
            Mode::Left => BorderType::Rounded,

            Mode::Hover => BorderType::Double
        })
    }

    #[inline]
    fn border_style(&self) -> Option<Style> {
        Some(match self.cursor_mode {
            Mode::Entered |
            Mode::Hover => Style::default().fg(Color::Blue),

            Mode::Left => Style::default()
        })
    }
}
