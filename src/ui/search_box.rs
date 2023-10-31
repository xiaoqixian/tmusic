// Date: Mon Oct 23 23:31:22 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::{
    layout::{Constraint, Rect}, 
    widgets::{Paragraph, Widget},
    text::Span
};

use crossterm::event::{Event, KeyCode};

use super::component::{
    CompState, 
    Component, 
    CompMode,
    Query,
    QueryResponse,
    Attribution
};

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
    fn query(&self, q: Query) -> QueryResponse {
        match q {
            Query::Title => QueryResponse::Title(Some(String::from("搜索栏"))),
            Query::Constraint => QueryResponse::Constraint(self.constraint.clone()),
            Query::UpdateDuration => QueryResponse::UpdateDuration(self.update_duration())
        }
    }

    fn set_attr(&mut self, attr: Attribution) -> Option<CompState> {
        match attr {
            Attribution::Area(area) => self.area = area,
            Attribution::Mode(mode) => self.mode = mode
        }
        None
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let text = Span::raw(
            match self.mode {
                CompMode::Enter => {
                    self.blink = !self.blink;
                    let mut display_str = self.input.clone();
                    
                    if self.blink {
                        display_str.push('|');
                    }

                    display_str
                },
                _ => String::new()
            }
        );

        Paragraph::new(text)
            .render(self.area, buffer);
    }

    fn feed_event(&mut self, event: crossterm::event::Event) -> CompState {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompState::Exit,
                _ => {}
            },
            _ => {}
        }

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

