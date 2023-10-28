// Date: Mon Oct 23 20:43:10 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use super::nested::Nested;
use tui::layout::Constraint;
use super::component::Component;

pub fn new() -> Nested {
    Nested::new(Constraint::Min(10))
}
