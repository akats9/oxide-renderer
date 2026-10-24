use pixels::Error;
use winit::event_loop::EventLoop;

pub use renderer::Renderer;
pub use app::App;

pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);
pub struct Point(pub usize, pub usize);

pub mod app;
pub mod renderer;