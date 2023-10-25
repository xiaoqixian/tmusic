// Date: Sat Oct 21 17:51:11 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    layout::{Constraint, Rect, Layout, Direction},
    widgets::{Widget, Clear},
    buffer::Buffer
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub struct Popup<T> {
    inner_widget: T,
    width_percentage: u16,
    height_percentage: u16
}

impl<T> Popup<T> {
    pub fn new(inner_widget: T, w: u16, h: u16) -> Self {
        Self {
            inner_widget,
            width_percentage: w,
            height_percentage: h,
        }
    }
}

impl<T> Widget for Popup<T> 
where T: Widget
{
    // popup window size is based on the view port size
    fn render(self, view_port: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(
            self.width_percentage, 
            self.height_percentage, 
            view_port
        );

        // clear the popup area
        Clear.render(popup_area, buf);
        self.inner_widget.render(popup_area, buf);
    }
}
