//! Game module

use crate::Renderer;

/// Provides an interface for updating game state and 
/// drawing the result to screen.
pub trait Game {
    fn update(&mut self, dt: f32);
    fn draw(&mut self, renderer: &mut Renderer);
}