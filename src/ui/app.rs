// Date: Mon Oct 23 20:43:10 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    rc::Rc,
    cell::RefCell
};

use tui::{
    layout::{Direction, Constraint, Rect, Layout},
    buffer::Buffer
};

use crossterm::event::{Event, KeyCode};

use super::component::{Component, CompMode, Motion};

pub struct App<C> {
    comp_stack: RefCell<Vec<C>>,
    components: Vec<C>,
    direction: Direction,
    margin: u16,
    hover_index: RefCell<usize>
}

impl<C> App<C> {
    pub fn new() -> Self {
        Self {
            comp_stack: RefCell::new(Vec::new()),
            components: Vec::new(),
            direction: Direction::Vertical,
            margin: 1,
            hover_index: RefCell::new(0)
        }
    }
}

impl App<Rc<dyn Component>> {
    #[inline]
    pub fn registrate(&mut self, comp: Rc<dyn Component>) {
        self.components.push(comp);
    }

    /// init method should only be called
    /// after all components are registrated
    pub fn init(&self, area: Rect) {
        self.set_area(area);
        // hover on the default first component
        assert!(*self.hover_index.borrow() < self.components.len());

        self.components[*self.hover_index.borrow()].hover();
    }
}

impl Component for App<Rc<dyn Component>> {
    fn set_area(&self, area: Rect) {
        let constraints = self.components
            .iter()
            .map(|comp| comp.get_constraint())
            .collect::<Vec<Constraint>>();

        let chunks = Layout::default()
            .direction(self.direction.clone())
            .margin(self.margin)
            .constraints(constraints)
            .split(area);

        self.components
            .iter()
            .enumerate()
            .for_each(|(index, comp)| {
                comp.set_area(chunks[index]);
            });
    }

    fn get_constraint(&self) -> Constraint {
        Constraint::Min(10)
    }

    fn read_event(&self, event: Event) -> CompMode<Rc<dyn Component>> {
        if self.components.is_empty() {
            return CompMode::Exit;
        }

        // firstly process global event
        let global_event = match event {
            Event::Resize(width, height) => {
                self.set_area(Rect::new( 
                    0,
                    0, 
                    width, 
                    height
                ));
                true
            },
            _ => false
        };

        if global_event {
            return CompMode::Stay;
        }

        let top = self.comp_stack.borrow_mut().pop();
        
        if let Some(top_comp) = top {
            match top_comp.read_event(event) {
                CompMode::Stay => {
                    self.comp_stack.borrow_mut().push(top_comp);
                }

                CompMode::Enter(new_top_comp) => {
                    self.comp_stack.borrow_mut().push(top_comp);
                    self.comp_stack.borrow_mut().push(new_top_comp);
                },

                // if exit, then the component back to hover mode
                CompMode::Exit => {
                    top_comp.hover();
                }
            }

            return CompMode::Stay;
        } 

        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => return CompMode::Exit,

                // process cursor moving keys
                KeyCode::Up | KeyCode::Down |
                KeyCode::Left | KeyCode::Right |
                KeyCode::Char('h' | 'j' | 'k' | 'l') => {
                    let curr = *self.hover_index.borrow();
                    *self.hover_index.borrow_mut() =
                        self.jump(curr, key_event.code);
                },

                KeyCode::Enter => {
                    let hover_comp = match self.components
                        .get(*self.hover_index.borrow()) {
                            Some(comp) => comp.clone(),
                            None => panic!(
                                "component at hover index {} not found", 
                                *self.hover_index.borrow()
                            )
                    };

                    hover_comp.enter();
                    self.comp_stack
                        .borrow_mut()
                        .push(hover_comp);
                }
                _ => {}
            },
            _ => {}
        }
        
        CompMode::Stay
    }

    fn render(&self, buffer: &mut Buffer) {
        self.components
            .iter()
            .for_each(|comp| comp.render(buffer));
    }

    #[inline]
    fn inner_components_size(&self) -> usize {
        self.components.len()
    }

    #[inline]
    fn direction(&self) -> Option<Direction> {
        Some(self.direction.clone())
    }
}
