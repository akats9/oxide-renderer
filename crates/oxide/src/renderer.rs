//! Renderer module.

use pixels::{Pixels, Error};
use winit::dpi::PhysicalSize;
use crate::{Point, ColorRGBA};

/// Provides methods for rendering to the screen.
/// IMPORTANT: draw_*() methods on Renderer should be used for drawing simple static objects and scenes.
/// For motion and layering use Sprites.
#[derive(Default)]
pub struct Renderer {
    pixels: Option<Pixels<'static>>,
    screen_width: usize,
    screen_height: usize,
}

impl Renderer {
    /// Create a new instance of Renderer.
    pub(crate) fn new(pixels: Pixels<'static>, w: usize, h: usize) -> Self {
        Self {
            pixels: Some(pixels),
            screen_width: w,
            screen_height: h,
        }
    }

    /// Render pixels.
    pub(crate) fn render(&mut self) -> Result<(), Error> {
        if let Some(pixels) = self.pixels.as_mut() {
            pixels.render()?;
        }
        Ok(())
    }

    /// Resize window.
    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) -> Result<(), Error> {
        if let Some(pixels) = self.pixels.as_mut() {
            pixels.resize_surface(size.width, size.height)?;
        }
        Ok(())
    }
}

impl Renderer {
    /// Draw a single pixel with specified color.
    pub fn draw_pixel(&mut self, point: Point, color: ColorRGBA) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        let Point(x, y) = point;
        if x >= self.screen_width || y >= self.screen_height {
            return;
        }

        let idx = (y * self.screen_width + x) * 4;
        frame[idx + 0] = color.0;
        frame[idx + 1] = color.1;
        frame[idx + 2] = color.2;
        frame[idx + 3] = color.3;
    }

    /// Clear entire screen to specified color.
    pub fn clear(&mut self, clear: ColorRGBA) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = clear.0;
            pixel[1] = clear.1;
            pixel[2] = clear.2;
            pixel[3] = clear.3;
        }
    }
}

mod line;
mod shapes;
pub(crate) mod sprites;