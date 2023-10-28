// Date: Thu Oct 26 10:38:17 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    widgets::Widget,
    layout::{Rect, Constraint}
};
use super::component::{CompMode, Component};

/// SingleWidget is a simple wrapper of 
/// a type that implements tui::widgets::Widget
/// to make it compatible with the Component trait.
pub struct SingleWidget<W> {
    widget: W,
    area: Rect,
    constraint: Constraint
}

impl<W> SingleWidget<W>
where W: Widget {
    pub fn new(widget: W, c: Constraint) -> Self {
        Self {
            widget,
            area: Default::default(),
            constraint: c
        }
    }
}

impl<W> Component for SingleWidget<W>
where W: Widget + Clone
{
    #[inline]
    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    fn read_event(&mut self, event: crossterm::event::Event) -> CompMode {
        CompMode::Exit
    }

    fn render(&self, buffer: &mut tui::buffer::Buffer) {
        self.widget
            .clone()
            .render(self.area, buffer);
    }
}
