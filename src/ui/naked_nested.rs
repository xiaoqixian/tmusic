// Date: Mon Oct 30 16:58:32 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use crossterm::event::{Event, KeyCode};
use tui::layout::{Rect, Direction, Layout, Constraint};

use super::component::{
    CompState, 
    Component, 
    CompMode,
    Query,
    QueryResponse,
    Attribution
};

#[derive(Debug, Clone, Copy)]
enum CursorMode {
    Entered(usize),
    Hover(usize)
}

/// Provide a naked nested component.
/// A naked nested component is just like a nested
/// component, but the cursor falls in it when the
/// cursor hovers it.
/// This component is provided to enable components
/// aligned in a different direction than the direction
/// of the its parent nested component.
pub struct NakedNested {
    inner_comps: Vec<Box<dyn Component>>,
    cursor: CursorMode,
    constraint: Constraint,
    area: Option<Rect>,
    direction: Direction, //vertical by default
}

impl NakedNested {
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

    // [WARN]: if a component fall on hover and exit on entered
    // may create an infinite loop and cause stack overflow.
    fn hover(&mut self, comp_index: usize) {
        let comp_state = match self.inner_comps[comp_index].alter_mode(CompMode::Hover) {
            Some(cs) => cs,
            None => return
        };
        match comp_state {
            CompState::Fall => {
                self.enter(comp_index);
            },
            _ => {
                self.cursor = CursorMode::Hover(comp_index);
            }
        }
    }

    fn enter(&mut self, comp_index: usize) {
        let comp_state = match self.inner_comps[comp_index].alter_mode(CompMode::Hover) {
            Some(cs) => cs,
            None => return
        };
        match comp_state {
            CompState::Exit => {
                self.hover(comp_index);
            },
            _ => {
                self.cursor = CursorMode::Entered(comp_index);
            }
        }
    }

    fn leave(&mut self, old_index: usize, new_index: usize) {
        if old_index != new_index {
            self.inner_comps[old_index]
                .alter_mode(CompMode::Leave);
            self.inner_comps[new_index]
                .alter_mode(CompMode::Hover);
            self.cursor = CursorMode::Hover(new_index);
        }
    }
}

impl Component for NakedNested {
    fn query(&self, q: Query) -> QueryResponse {
        match q {
            Query::Title => QueryResponse::Title(None),
            Query::Constraint => QueryResponse::Constraint(self.constraint.clone()),
            Query::UpdateDuration => {
                QueryResponse::UpdateDuration(self.update_duration())
            }
        }
    }

    fn set_attr(&mut self, attr: Attribution) -> Option<CompState> {
        match attr {
            Attribution::Area(area) => {
                self.set_area(area);
                None
            },
            Attribution::Mode(mode) => {
                self.alter_mode(mode)
            }
        }
    }

    fn feed_event(&mut self, event: Event) -> CompState {
        if self.inner_comps.is_empty() {
            return CompState::Stay;
        }

        let hover_index = match self.cursor {
            CursorMode::Entered(entered_index) => {
                match self.inner_comps
                    .get_mut(entered_index)
                    .unwrap()
                    .feed_event(event.clone()) {
                    CompState::Exit => {
                        self.hover(entered_index);
                        return CompState::Stay;
                    },

                    CompState::ExitIgnore => {
                        // rehover may create an infinite loop
                        entered_index
                    },

                    _ => return CompState::Stay
                }
            },
            CursorMode::Hover(i) => i
        };

        let len = self.inner_comps.len();
        let vertical = match self.direction {
            Direction::Vertical => true,
            Direction::Horizontal => false
        };

        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => return CompState::ExitIgnore,
                KeyCode::Enter => {
                    if let Some(comp_state) = self.inner_comps[hover_index]
                        .alter_mode(CompMode::Enter)
                    {
                        match comp_state {
                            CompState::Exit => {},
                            _ => self.cursor = CursorMode::Entered(hover_index)
                        }
                    }
                },
                KeyCode::Up | KeyCode::Char('k') if vertical => {
                    let new_hover = std::cmp::max(1, hover_index) - 1;
                    self.leave(hover_index, new_hover);
                }
                KeyCode::Left | KeyCode::Char('h') if !vertical => {
                    let new_hover = std::cmp::max(1, hover_index) - 1;
                    self.leave(hover_index, new_hover);
                },
                KeyCode::Right | KeyCode::Char('l') if !vertical => {
                    let new_hover = std::cmp::min(len-1, hover_index+1);
                    self.leave(hover_index, new_hover);
                }
                KeyCode::Down | KeyCode::Char('j') if vertical => {
                    let new_hover = std::cmp::min(len-1, hover_index+1);
                    self.leave(hover_index, new_hover);
                }
                _ => {
                    return CompState::ExitIgnore;
                }
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

    fn set_area(&mut self, area: Rect) {
        if let Some(curr_area) = self.area {
            if curr_area == area {
                return;
            }
        }

        self.area = Some(area);
        self.realign();
    }

    /// This method is the key difference between Nested and
    /// NakedNested
    fn alter_mode(&mut self, mode: CompMode) -> Option<CompState> {
        match mode {
            CompMode::Hover => Some(CompState::Fall),
            CompMode::Enter => {
                if let CursorMode::Hover(hover_index) = self.cursor {
                    self.hover(hover_index);
                }
                None
            }
            CompMode::Leave => {
                self.inner_comps
                    .iter_mut()
                    .for_each(|comp| {
                        let _ = comp.alter_mode(CompMode::Leave);
                    });
                None
            }
        }
    }

    #[inline]
    fn update_duration(&self) -> Option<std::time::Duration> {
        match self.inner_comps
                    .iter()
                    .min_by(|comp1, comp2| {
                        let d1 = comp1.update_duration()
                            .unwrap_or(std::time::Duration::MAX);
                        let d2 = comp2.update_duration()
                            .unwrap_or(std::time::Duration::MAX);
                        d1.cmp(&d2)
                    }) 
        {
            None => None,
            Some(d) => d.update_duration()
        }
    }
}
