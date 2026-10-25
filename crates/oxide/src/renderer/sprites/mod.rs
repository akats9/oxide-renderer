//! Sprites module.

use crate::{ColorRGBA, Point, Renderer};

/// Velocity of a Sprite in x and y axes.
#[derive(Clone, Copy)]
pub struct Velocity(isize, isize);

/// Shape of a Sprite.
/// For syntax and performance reasons, 
/// basic shapes (Rectangles, Triangles and Circles) have
/// specific implementations of Sprite. Arbitrary shapes 
/// (loaded from assets or otherwise) have a generic implementation.
/// This enum therefore represents the four types of Sprite that are 
/// provided by Oxide. Implementor-defined Sprite objects are also 
/// possible and should use `Shape::Arbitrary`.
pub enum Shape {
    Rect,
    Triangle, 
    Circle,
    Arbitrary,
}

/// Defines required methods for Sprite objects that 
/// Oxide can work with to render complex scenes
/// as well as animations.
/// 
/// Sprite has `Default` as a supertrait to ensure that
/// Sprites can be constructed with the common syntax:
/// `foo::default()` and have their fields filled by
/// setter functions (trait and type-specific for different shapes).
pub trait Sprite : Default {

    // ====== GETTERS ======

    /// Get the position of the Sprite.
    fn position(&self) -> Point;
    
    /// Get the velocity of the Sprite.
    fn velocity(&self) -> Velocity;
    
    /// Get the color of the Sprite.
    fn color(&self) -> ColorRGBA;
    
    /// Get the z-index (i.e. Layer) of the Sprite.
    fn z_index(&self) -> usize;
    
    /// Should the Sprite be shown?
    fn shown(&self) -> bool;

    // ====== SETTERS ======
    
    /// Set the position of the Sprite.
    fn set_position(&mut self, p: Point);

    /// Set the velocity of the Sprite.
    fn set_velocity(&mut self, v: Velocity);

    /// Set the color of the Sprite.
    fn set_color(&mut self, c: ColorRGBA);

    /// Set the z-index (i.e. Layer) of the Sprite.
    fn set_z_index(&mut self, z: usize);

    /// Set if the Sprite should be shown.
    fn set_shown(&mut self, s: bool);

    // ====== RENDERING ======

    /// Get the Shape of the Sprite.
    fn shape(&self) -> Shape;

    /// Check if the Sprite is within a bounding box.
    /// Useful for determining if a Sprite should be redrawn when another Sprite moves,
    /// improving performance.
    fn is_in_bounding_box(&self, b: Box) -> bool;

    /// Draw the Sprite.
    fn draw(&mut self, r: &mut Renderer);

    /// Clear the Sprite from screen.
    /// `bg` represents the color of the background.
    /// Returns a bounding box that needs to be rerendered.
    fn clear(&mut self, r: &mut Renderer, bg: ColorRGBA) -> Box;

    /// Move a Sprite from its current position to a specified position
    /// in specified duration (in timesteps)
    fn animate(&mut self, end_pos: Point, duration: usize);

}

/// Helper type to represent bounding boxes and other helper rects.
/// Created from the endpoints of its diagonal.
pub struct Box(Point, Point);

pub mod rect;