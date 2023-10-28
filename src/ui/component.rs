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

use tui::{
    layout::{Constraint, Rect},
    buffer::Buffer,
};

use super::block::Block;

#[derive(Debug, Clone, Copy)]
pub enum CompState {
    Stay,
    Exit
}

#[derive(Debug, Clone, Copy)]
pub enum CompMode {
    Enter,
    Hover,
    Leave
}

pub trait Component {
    /// Every component should be confined in a certain area
    fn set_area(&mut self, area: Rect);

    /// Every component is supposed have a constraint
    fn get_constraint(&self) -> Constraint;

    fn read_event(&mut self, event: crossterm::event::Event) -> CompState;

    fn render(&mut self, buffer: &mut Buffer);

    fn alter_mode(&mut self, mode: CompMode) -> CompState;

    fn update_duration(&self) -> Option<std::time::Duration>;


    // Below are all built-in components
    // for you to easily wrap a component in.

    /// To wrap in a block
    #[inline]
    fn block(self) -> Block<Self>
    where Self: Sized
    {
        Block::new(self)
    }

    /// To wrap in a block with title
    #[inline]
    fn block_with_title(self, title: String) -> Block<Self>
    where Self: Sized
    {
        Block::new(self)
            .title(title)
    }
}

/// A stateful component is a component 
/// that carries a CompMode member.
pub trait StatefulComponent {
    fn comp_mode(&self) -> CompMode;
}

/// Provide a default border type/style getter
/// for stateful components that need a frame
/// whose border type/style is able to alternate
/// with the component state.
use tui::{
    widgets::BorderType,
    style::{Style, Color}
};
pub trait DefaultFrameStyle {
    fn border_type(&self) -> BorderType;

    fn border_style(&self) -> Style;
}

impl<C> DefaultFrameStyle for C
where C: Component + StatefulComponent
{
    fn border_type(&self) -> BorderType {
        match self.comp_mode() {
            CompMode::Enter |
            CompMode::Leave => BorderType::Rounded,

            CompMode::Hover => BorderType::Double
        }
    }

    fn border_style(&self) -> Style {
        Style::default().fg(
            match self.comp_mode() {
                CompMode::Enter |
                CompMode::Hover => Color::Blue,

                CompMode::Leave => Color::White
            }
        )
    }
}
