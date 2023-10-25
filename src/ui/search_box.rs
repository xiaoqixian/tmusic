// Date: Mon Oct 23 23:31:22 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::cell::RefCell;
use std::rc::Rc;

use tui::{
    layout::{Constraint, Direction, Rect}, 
    widgets::{Block, Borders, Widget}
};

use crossterm::event::{Event, KeyCode};

use super::component::{CompMode, Component, CursorMode, FrameStyle};

pub struct SearchBox {
    constraint: Constraint,
    area: RefCell<Rect>,
    cursor: RefCell<CursorMode>
}

impl SearchBox {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c,
            area: RefCell::new(Rect::default()),
            cursor: RefCell::new(CursorMode::Leave)
        }
    }
}

impl Component for SearchBox {
    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint
    }

    #[inline]
    fn set_area(&self, area: tui::layout::Rect) {
        *self.area.borrow_mut() = area;
    }

    fn render(&self, buffer: &mut tui::buffer::Buffer) {
        Block::default()
            .title("搜索栏")
            .borders(Borders::ALL)
            .border_type(self.get_border_type())
            .border_style(self.get_border_style())
            .render(self.area.borrow().clone(), buffer);
    }

    fn read_event(&self, event: crossterm::event::Event) -> 
        CompMode<Rc<dyn Component>> 
    {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompMode::Exit,
                _ => {}
            },
            _ => {}
        }

        CompMode::Stay
    }

    #[inline]
    fn inner_components_size(&self) -> usize {
        0
    }

    #[inline]
    fn direction(&self) -> Option<Direction> {
        None
    }

    #[inline]
    fn get_cursor(&self) -> CursorMode {
        self.cursor.borrow().clone()
    }
    
    #[inline]
    fn enter(&self) {
        *self.cursor.borrow_mut() = CursorMode::Entered;
    }

    #[inline]
    fn hover(&self) {
        *self.cursor.borrow_mut() = CursorMode::Hover;
    }

    #[inline]
    fn leave(&self) {
        *self.cursor.borrow_mut() = CursorMode::Leave;
    }
}

