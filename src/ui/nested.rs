// Date: Wed Oct 25 20:01:58 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use super::component::{CompState, Component, CompMode};

use crossterm::event::{Event, KeyCode};
use tui::layout::{Rect, Direction, Layout, Constraint};

#[derive(Debug, Clone, Copy)]
enum CursorMode {
    Entered(usize),
    Hover(usize)
}

pub struct Nested {
    inner_comps: Vec<Box<dyn Component>>,
    cursor: CursorMode,
    constraint: Constraint,
    area: Option<Rect>,
    direction: Direction, //vertical by default
}

impl Nested {
    pub fn new(c: Constraint) -> Self {
        Self {
            inner_comps: Vec::new(),
            cursor: CursorMode::Hover(0),
            constraint: c,
            area: None,
            direction: Direction::Vertical,
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn registrate<T>(&mut self, mut comp: T) 
    where T: Component + Sized + 'static
    {
        if self.inner_comps.is_empty() {
            comp.alter_mode(CompMode::Hover);
        }

        self.inner_comps
            .push(Box::from(comp));

        // if area is decided, registrate a new comp
        // need a whole new layout adjustment.
        self.realign();
    }

    fn realign(&mut self) {
        let area = match self.area {
            None => return,
            Some(area) => area
        };

        let constraints = self.inner_comps
            .iter()
            .map(|comp| comp.get_constraint())
            .collect::<Vec<Constraint>>();

        let chunks = Layout::default()
            .direction(self.direction.clone())
            .constraints(constraints)
            .split(area);

        self.inner_comps
            .iter_mut()
            .enumerate()
            .for_each(|(index, comp)| {
                comp.set_area(chunks[index])
            });
    }

}

impl Component for Nested {
    fn set_area(&mut self, area: Rect) {
        if let Some(curr_area) = self.area {
            if curr_area == area {
                return;
            }
        }

        self.area = Some(area);
        self.realign();
    }

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    fn read_event(&mut self, event: Event) -> CompState {
        if self.inner_comps.is_empty() {
            return CompState::Stay;
        }

        let hover_index = match self.cursor {
            CursorMode::Entered(entered_index) => {
                if let CompState::Exit = self.inner_comps
                    .get_mut(entered_index)
                    .unwrap() 
                    .read_event(event) {

                    self.cursor = CursorMode::Hover(entered_index);

                    self.inner_comps[entered_index]
                        .alter_mode(CompMode::Hover);
                }

                return CompState::Stay;
            },
            CursorMode::Hover(i) => i
        };

        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompState::Exit,
                KeyCode::Enter => {
                    if let Some(comp) = self.inner_comps.get_mut(hover_index) {
                        match comp.alter_mode(CompMode::Enter) {
                            CompState::Exit => {},
                            _ => {
                                self.cursor = CursorMode::Entered(hover_index);
                            }
                        }
                    }
                },

                KeyCode::Up | KeyCode::Char('k') |
                KeyCode::Down | KeyCode::Char('j') |
                KeyCode::Left | KeyCode::Char('h') |
                KeyCode::Right | KeyCode::Char('l') => {
                    let len = self.inner_comps.len();

                    let new_hover = {
                        match key_event.code {
                            KeyCode::Up | KeyCode::Char('k') |
                            KeyCode::Left | KeyCode::Char('h') =>
                                std::cmp::max(1, hover_index) - 1,
                            KeyCode::Down | KeyCode::Char('j') |
                            KeyCode::Right | KeyCode::Char('l') =>
                                std::cmp::min(len-1, hover_index+1),
                            _ => hover_index // no likely to happen
                        }
                    };
                
                    if new_hover != hover_index {
                        self.inner_comps[hover_index]
                            .alter_mode(CompMode::Leave);
                        self.inner_comps[new_hover]
                            .alter_mode(CompMode::Hover);
                    }

                    self.cursor = CursorMode::Hover(new_hover);
                }
                _ => {}
            },
            _ => {}
        }

        CompState::Stay
    }

    #[inline]
    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        self.inner_comps
            .iter_mut()
            .for_each(|comp| comp.render(buffer));
    }

    #[inline]
    fn alter_mode(&mut self, mode: CompMode) -> CompState {
        CompState::Stay
    }

    #[inline]
    fn update_duration(&self) -> Option<std::time::Duration> {
        if self.inner_comps.is_empty() {
            return None;
        }
        self.inner_comps
            .iter()
            .min_by(|comp1, comp2| {
                let d1 = comp1.update_duration()
                    .unwrap_or(std::time::Duration::MAX);
                let d2 = comp2.update_duration()
                    .unwrap_or(std::time::Duration::MAX);
                d1.cmp(&d2)
            })
            .unwrap()
            .update_duration()
    }
}
