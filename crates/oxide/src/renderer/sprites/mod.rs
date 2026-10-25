//! Sprites module.

// ====== RE-EXPORTS ======
pub use rect::Rect;
pub use circle::Circle;
pub use triangle::Triangle;

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

/// Abstraction over the types of Sprites.

pub enum AnySprite {
    Rect(Rect),
    Triangle(Triangle),
    Circle(Circle),
}

impl Default for AnySprite {
    fn default() -> Self {
        Self::Rect(Rect::default())
    }
}

macro_rules! delegate_sprite {
    ($($variant:ident),+ $(,)?) => {
        impl Sprite for AnySprite {
            fn position(&self) -> Point {
                match self {
                    $(Self::$variant(s) => s.position(),)+
                }
            }

            fn velocity(&self) -> Velocity {
                match self {
                    $(Self::$variant(s) => s.velocity(),)+
                }
            }

            fn color(&self) -> ColorRGBA {
                match self {
                    $(Self::$variant(s) => s.color(),)+
                }
            }

            fn z_index(&self) -> usize {
                match self {
                    $(Self::$variant(s) => s.z_index(),)+
                }
            }

            fn shown(&self) -> bool {
                match self {
                    $(Self::$variant(s) => s.shown(),)+
                }
            }

            fn set_position(&mut self, p: Point) {
                match self {
                    $(Self::$variant(s) => s.set_position(p),)+
                }
            }

            fn set_velocity(&mut self, v: Velocity) {
                match self {
                    $(Self::$variant(s) => s.set_velocity(v),)+
                }
            }

            fn set_color(&mut self, c: ColorRGBA) {
                match self {
                    $(Self::$variant(s) => s.set_color(c),)+
                }
            }

            fn set_z_index(&mut self, z: usize) {
                match self {
                    $(Self::$variant(s) => s.set_z_index(z),)+
                }
            }

            fn set_shown(&mut self, shown: bool) {
                match self {
                    $(Self::$variant(s) => s.set_shown(shown),)+
                }
            }

            fn shape(&self) -> Shape {
                match self {
                    $(Self::$variant(s) => s.shape(),)+
                }
            }

            fn is_in_bounding_box(&self, b: Box) -> bool {
                match self {
                    $(Self::$variant(s) => s.is_in_bounding_box(b),)+
                }
            }

            fn draw(&mut self, r: &mut Renderer) {
                match self {
                    $(Self::$variant(s) => s.draw(r),)+
                }
            }

            fn clear(&mut self, r: &mut Renderer, bg: ColorRGBA) -> Box {
                match self {
                    $(Self::$variant(s) => s.clear(r, bg),)+
                }
            }
        }
    };
}

delegate_sprite!(Rect, Triangle, Circle);

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

}

/// Helper type to represent bounding boxes and other helper rects.
/// Created from the endpoints of its diagonal.
pub struct Box(Point, Point);

impl Box {
    /// Check if point is inside box.
    fn contains(&self, p: Point) -> bool {
        let Point(x,y) = p;
        let Point(bx1, by1) = self.0;
        let Point(bx2, by2) = self.1;
        if is_between(x, bx1, bx2) && is_between(y, by1, by2) { return true; }
        false
    }

    /// Check if box is inside box.
    fn contains_box(&self, b: Box) -> bool {
        if self.contains(b.0) && self.contains(b.1) { return true; }
        false
    }
}

fn is_between(v: usize, a: usize, b: usize) -> bool {
    let min = std::cmp::min(a, b);
    let max = std::cmp::max(a, b);
    (min..=max).contains(&v)
}

pub(crate) mod rect;
pub(crate) mod triangle;
pub(crate) mod circle;