// Date: Fri Oct 27 23:35:38 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use tui::layout::Constraint;

use super::component::{
    CompState, 
    Component, 
    Query,
    QueryResponse,
    Attribution
};

pub struct WhitePanel {
    constraint: Constraint
}

impl WhitePanel {
    pub fn new(c: Constraint) -> Self {
        Self {
            constraint: c
        }
    }
}

impl Component for WhitePanel {
    fn query(&self, q: Query) -> QueryResponse {
        match q {
            Query::Constraint => 
                QueryResponse::Constraint(self.constraint.clone()),
            Query::Title => QueryResponse::Title(None),
            Query::UpdateDuration => QueryResponse::UpdateDuration(None)
        }
    }

    #[inline]
    fn set_attr(&mut self, _: Attribution) -> Option<CompState> {
        None
    }

    #[inline]
    fn feed_event(&mut self, _: crossterm::event::Event) -> CompState {
        CompState::Stay
    }

    #[inline]
    fn render(&mut self, _: &mut tui::buffer::Buffer) {
        
    }
}
