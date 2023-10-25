// Date: Wed Oct 25 20:01:58 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use super::component::{CompMode, Component};

use crossterm::event::{Event, KeyCode};
use tui::layout::{Rect, Direction, Layout, Constraint};

#[derive(Debug, Clone, Copy)]
enum Mode {
    Entered(usize),
    Hover(usize)
}

pub struct Nested {
    inner_comps: Vec<Box<dyn Component>>,
    cursor: Mode,
    constraint: Constraint,
    area: Option<Rect>,
    direction: Direction, //vertical by default
}

impl Nested {
    pub fn new(c: Constraint) -> Self {
        Self {
            inner_comps: Vec::new(),
            cursor: Mode::Hover(0),
            constraint: c,
            area: None,
            direction: Direction::Vertical,
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn registrate(&mut self, comp: Box<dyn Component>) {
        self.inner_comps
            .push(comp);

        // if area is decided, registrate a new comp
        // need a whole new layout adjustment.
        if let Some(area) = self.area {
            self.set_area(area);
        }
    }
}

impl Component for Nested {
    fn set_area(&mut self, area: Rect) {
        self.area = Some(area);

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

    #[inline]
    fn get_constraint(&self) -> Constraint {
        self.constraint.clone()
    }

    fn read_event(&mut self, event: Event) -> CompMode {
        let hover_index = match self.cursor {
            Mode::Entered(entered_index) => {
                if let CompMode::Exit = self.inner_comps
                    .get_mut(entered_index)
                    .unwrap() 
                    .read_event(event) {

                    self.cursor = Mode::Hover(entered_index);
                }

                return CompMode::Stay;
            },
            Mode::Hover(i) => i
        };

        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompMode::Exit,
                KeyCode::Enter => {
                    if self.inner_comps[hover_index].is_enterable() {
                        self.inner_comps[hover_index].enter();
                        self.cursor = Mode::Entered(hover_index);
                    }
                },

                KeyCode::Up | KeyCode::Char('k') |
                KeyCode::Down | KeyCode::Char('j') |
                KeyCode::Left | KeyCode::Char('h') |
                KeyCode::Right | KeyCode::Char('l') => {
                    let len = self.inner_comps.len();

                    self.cursor = Mode::Hover({
                        match key_event.code {
                            KeyCode::Up | KeyCode::Char('k') |
                            KeyCode::Left | KeyCode::Char('h') =>
                                std::cmp::max(0, hover_index-1),
                            KeyCode::Down | KeyCode::Char('j') |
                            KeyCode::Right | KeyCode::Char('l') =>
                                std::cmp::min(len-1, hover_index+1),
                            _ => hover_index // no likely to happen
                        }
                    });
                }
                _ => {}
            },
            _ => {}
        }

        CompMode::Stay
    }

    #[inline]
    fn render(&self, buffer: &mut tui::buffer::Buffer) {
        self.inner_comps
            .iter()
            .for_each(|comp| comp.render(buffer));
    }
}
