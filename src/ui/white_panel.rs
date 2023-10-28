// Date: Fri Oct 27 23:35:38 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::layout::Constraint;

use super::component::{
    CompState,
    Component,
    CompMode
};

pub struct WhitePanel {
    constraint: Constraint
}

impl WhitePanel {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c
        }
    }
}

impl Component for WhitePanel {
    #[inline]
    fn set_area(&mut self, _: tui::layout::Rect) {}

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    #[inline]
    fn render(&mut self, _: &mut tui::buffer::Buffer) {}

    #[inline]
    fn read_event(&mut self, _: crossterm::event::Event) -> CompState {
        CompState::Exit
    }

    #[inline]
    fn alter_mode(&mut self, _: CompMode) -> CompState {
        CompState::Exit
    }

    #[inline]
    fn update_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
