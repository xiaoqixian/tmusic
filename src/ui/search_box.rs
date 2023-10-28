// Date: Mon Oct 23 23:31:22 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    layout::{Constraint, Rect}, 
    widgets::{Paragraph, Widget},
    text::Span
};

use crossterm::event::{Event, KeyCode};

use super::component::{CompMode, Component, CompState};

pub struct SearchBox {
    constraint: Constraint,
    area: Rect,
    mode: CompMode,
    input: String,
    blink: bool
}

impl SearchBox {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c,
            area: Rect::default(),
            mode: CompMode::Leave,
            input: String::new(),
            blink: false
        }
    }
}

impl Component for SearchBox {
    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    #[inline]
    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let text = Span::raw(
            match self.mode {
                CompMode::Enter => {
                    if self.input.is_empty() {
                        self.blink = !self.blink;
                        if self.blink {
                            " "
                        } else {
                            "|"
                        }
                    } else {
                        self.input.as_str()
                    }
                },
                _ => " "
            }
        );

        Paragraph::new(text)
            .render(self.area, buffer);
    }

    fn read_event(&mut self, event: crossterm::event::Event) -> CompState {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompState::Exit,
                _ => {}
            },
            _ => {}
        }

        CompState::Stay
    }

    #[inline]
    fn alter_mode(&mut self, mode: CompMode) -> CompState {
        self.mode = mode;
        CompState::Stay
    }

    fn update_duration(&self) -> Option<std::time::Duration> {
        match self.mode {
            CompMode::Enter => {
                if self.input.is_empty() {
                    Some(std::time::Duration::from_millis(500))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

