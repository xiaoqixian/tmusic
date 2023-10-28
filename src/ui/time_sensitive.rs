// Date: Thu Oct 26 17:51:40 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

/// Provide a time sensitive component
/// A time sensitive component is such a 
/// component that it must be rendered every
/// `duration` of time.

use std::time::Duration;

use super::component::{CompMode, Component};

pub struct TimeSensitive<C> {
    inner: C,
    duration: Duration
}

impl<C> TimeSensitive<C> {
    pub fn new(inner: C, duration: Duration) -> Self {
        Self {
            inner,
            duration
        }
    }
}

impl<C> Component for TimeSensitive<C>
where C: Component
{
    #[inline]
    fn set_area(&mut self, area: tui::layout::Rect) {
        self.inner.set_area(area)
    }

    #[inline]
    fn get_constraint(&self) -> tui::layout::Constraint {
        self.inner.get_constraint()
    }

    #[inline]
    fn is_enterable(&self) -> bool {
        self.inner.is_enterable()
    }

    #[inline]
    fn render(&self, buffer: &mut tui::buffer::Buffer) {
        self.inner.render(buffer)
    }

    #[inline]
    fn read_event(&mut self, event: crossterm::event::Event) -> CompMode {
        self.inner.read_event(event)
    }
}
