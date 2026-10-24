//! Oxide: Software renderer and mini game engine.

// ====== RE-EXPORTS ======
pub use renderer::Renderer;
pub use app::App;
pub use game::Game;
pub use renderer::sprites::Sprite;

/// Represents a 4 channel RGBA color with 4 bytes per pixel
#[derive(Clone, Copy)]
pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

/// Represents a single point on the screen. Origin is at top-left.
#[derive(Clone, Copy)]
pub struct Point(pub usize, pub usize);

mod app;
mod renderer;
mod game;