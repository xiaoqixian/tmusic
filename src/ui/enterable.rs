// Date: Wed Oct 25 21:05:42 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    widgets::BorderType,
    style::{Style, Color}
};

use crossterm::event::Event;

use super::component::{CompMode, Component, ComponentWrapper};

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

impl<C> ComponentWrapper for Enterable<C>
where C: Component
{
    type Inner = C;

    #[inline]
    fn inner_ref<'a>(&'a self) -> Option<&'a Self::Inner> {
        Some(&self.inner)
    }

    #[inline]
    fn inner_mut<'a>(&'a mut self) -> Option<&'a mut Self::Inner> {
        Some(&mut self.inner)
    }
}

impl<C> Enterable<C>
where C: Component
{
    #[inline]
    pub fn read_event(&mut self, event: Event) -> CompMode {
        let inner_mode = self.inner.read_event(event);
        if let CompMode::Exit = inner_mode {
            self.cursor_mode = Mode::Hover;
        }
        inner_mode
    }

    /// @Override
    #[inline]
    pub fn is_enterable(&self) -> bool {
        true
    }

    #[inline]
    pub fn enter(&mut self) {
        self.cursor_mode = Mode::Entered;
        self.inner.enter();
    }

    #[inline]
    pub fn hover(&mut self) {
        self.cursor_mode = Mode::Hover;
        self.inner.hover();
    }

    #[inline]
    pub fn leave(&mut self) {
        self.cursor_mode = Mode::Left;
        self.inner.leave();
    }

    #[inline]
    pub fn border_type(&self) -> BorderType {
        match self.cursor_mode {
            Mode::Entered |
            Mode::Left => BorderType::Rounded,

            Mode::Hover => BorderType::Double
        }
    }

    #[inline]
    pub fn border_style(&self) -> Style {
        match self.cursor_mode {
            Mode::Entered |
            Mode::Hover => Style::default().fg(Color::Blue),

            Mode::Left => Style::default()
        }
    }
}

