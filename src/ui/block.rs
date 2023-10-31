// Date: Chu Oct 26 10:25:47 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::layout::{Rect, Constraint};
use tui::widgets::{
    Widget, 
    Block as TuiBlock, 
    Borders, 
    BorderType
};
use tui::style::{Style, Color};

use super::component::{
    CompState, 
    Component, 
    CompMode,
    Query,
    QueryResponse,
    Attribution
};

pub struct Block<C> {
    inner: C,
    area: Option<Rect>,
    title: Option<String>,
    mode: CompMode,
    constraint: Constraint
}

impl<C> Block<C> {
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    fn border_type(&self) -> BorderType {
        match self.mode {
            CompMode::Enter |
            CompMode::Leave => BorderType::Rounded,

            CompMode::Hover => BorderType::Double
        }
    }

    fn border_style(&self) -> Style {
        match self.mode {
            CompMode::Enter |
            CompMode::Hover => Style::default().fg(Color::Blue),

            CompMode::Leave => Style::default().fg(Color::White)
        }
    }
}

impl<C> Block<C>
where C: Component
{
    pub fn new(inner: C) -> Self {
        Self {
            constraint: inner.get_constraint(),
            inner,
            title: Default::default(),
            area: None,
            mode: CompMode::Leave,
        }
    }

}

impl<C> Component for Block<C>
where C: Component
{
    fn query(&self, q: Query) -> QueryResponse {
        match q {
            Query::Constraint => 
                QueryResponse::Constraint(self.constraint.clone()),
            q => self.inner.query(q)
        }
    }

    fn set_attr(&mut self, attr: Attribution) -> Option<CompState> {
        match attr {
            Attribution::Area(area) => {
                self.set_area(area);
                None
            },
            Attribution::Mode(mode) => self.alter_mode(mode),
        }
    }

    #[inline]
    fn feed_event(&mut self, event: crossterm::event::Event) -> CompState {
        self.inner.feed_event(event)
    }

    fn render(&mut self, buffer: &mut tui::buffer::Buffer) {
        let area = match self.area {
            None => return,
            Some(area) => area
        };
        let block = TuiBlock::default()
            .borders(Borders::ALL)
            .border_style(self.border_style())
            .border_type(self.border_type())
            .title(
                self.title.as_ref()
                .map(|title| title.as_str())
                .unwrap_or("")
            );

        block.render(area, buffer);
        self.inner.render(buffer);
    }

    fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
        self.inner
            .set_area(
                TuiBlock::default()
                .borders(Borders::ALL)
                .inner(area)
            );
    }

    fn alter_mode(&mut self, mode: CompMode) -> Option<CompState> {
        self.mode = mode;

        match self.inner.alter_mode(mode) {
            None => None,
            Some(comp_state) => {
                match comp_state {
                    CompState::Exit => {
                        self.mode = CompMode::Hover;
                    },
                    _ => {}
                }
                Some(comp_state)
            }
        }
    }
}
