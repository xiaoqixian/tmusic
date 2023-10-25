// Date: Wed Oct 25 15:02:07 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::layout::{Constraint, Direction};

pub struct ProgressBar {
    constraint: Constraint,
    direction: Direction
}

impl ProgressBar {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c,
            direction: Direction::Horizontal
        }
    }
}
