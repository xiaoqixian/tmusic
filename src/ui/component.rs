// Date: Fri Oct 20 18:50:48 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

/*
 * Component should have a hover and enter method
 * When it's hovered on or entered in, it should 
 * alternate its widget style.
 *
 * There are two types of Component:
 *  1. One with inner components,
 *     represented with Vec<Vec<Component>>
 *  2. One that contains a Widget
 *
 * Every Component should also have a listen_key method,
 * when a Component is entered, it should take over the 
 * power to read key events. Until it transfers this power 
 * to its inner components, or exit `listen_key` method.
 */

use std::rc::Rc;
use crossterm::event::{Event, KeyCode};

use tui::{
    layout::{Constraint, Rect, Direction},
    buffer::Buffer,
    style::{Style, Color},
    widgets::BorderType
};

use super::enterable::Enterable;

#[derive(Clone, Debug)]
pub enum CursorMode {
    Hover,
    Entered,
    Leave
}

pub enum CompMode {
    Stay,
    Exit
}

pub trait Component {
    /// Every component should be confined in a certain area
    fn set_area(&mut self, area: Rect);

    /// Every component is supposed have a constraint
    fn get_constraint(&self) -> Constraint;

    fn read_event(&mut self, event: Event) -> CompMode;

    fn render(&self, buffer: &mut Buffer);

    #[inline]
    fn inner_components_size(&self) -> usize {0}

    /// For component to alternate style
    /// when entered, if necessary.
    fn enter(&mut self) {}

    /// Same as enter
    fn hover(&mut self) {}

    /// Same as enter
    fn leave(&mut self) {}

    /// Components are unenterable by default.
    /// But if they are wrapped in an Enterable type.
    /// This method will be override by Enterable, 
    /// and then they are enterable.
    #[inline]
    fn is_enterable(&self) -> bool { false }

    #[inline]
    fn enterable(self) -> Enterable<Self> 
    where Self: Sized
    {
        Enterable::new(self)
    }

    fn set_cursor(&self, cursor: CursorMode) {}

    #[inline]
    fn get_cursor(&self) -> CursorMode {
        CursorMode::Leave
    }

    #[inline]
    fn border_type(&self) -> Option<BorderType> {
        None
    }

    #[inline]
    fn border_style(&self) -> Option<Style> {
        None
    }
}

/*pub trait Motion {*/
    /*fn jump(&self, init: usize, key_code: KeyCode) -> usize;*/
/*}*/

/*impl<T> Motion for T*/
/*where T: Component*/
/*{*/
    /*fn jump(&self, mut init: usize, key_code: KeyCode) -> usize {*/
        /*let vertical = match self.direction() {*/
            /*None => return init,*/
            /*Some(Direction::Vertical) => true,*/
            /*Some(Direction::Horizontal) => false*/
        /*};*/

        /*let len = self.inner_components_size();*/

        /*match key_code {*/
            /*KeyCode::Up | KeyCode::Char('k') if vertical*/
                /*=> init = std::cmp::max(0, init-1),*/
            /*KeyCode::Down | KeyCode::Char('j') if vertical*/
                /*=> init = std::cmp::min(len-1, init+1),*/
            /*KeyCode::Left | KeyCode::Char('h') if !vertical*/
                /*=> init = std::cmp::max(0, init-1),*/
            /*KeyCode::Right | KeyCode::Char('l') if !vertical*/
                /*=> init = std::cmp::min(len-1, init+1),*/
            /*_ => {}*/
        /*}*/

        /*init*/
    /*}*/
/*}*/

/*pub trait FrameStyle {*/
    /*fn get_border_type(&self) -> BorderType; */

    /*fn get_border_style(&self) -> Style;*/
/*}*/

/*impl<T> FrameStyle for T*/
/*where T: Component */
/*{*/
    /*fn get_border_type(&self) -> BorderType {*/
        /*match self.get_cursor() {*/
            /*CursorMode::Hover =>*/
                /*BorderType::Double,*/
            /*CursorMode::Entered => */
                /*BorderType::Rounded,*/
            /*CursorMode::Leave => */
                /*BorderType::Rounded*/
        /*}*/
    /*}*/

    /*fn get_border_style(&self) -> Style {*/
        /*let mut style = Style::default();*/
        /*match self.get_cursor() {*/
            /*CursorMode::Hover => */
                /*style.fg(Color::Blue),*/
            /*CursorMode::Entered =>*/
                /*style.fg(Color::Blue),*/
            /*CursorMode::Leave =>*/
                /*style.fg(Color::White)*/
        /*}*/
    /*}*/
/*}*/
