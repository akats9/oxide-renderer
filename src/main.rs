use std::sync::Arc;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Default)]
struct App{
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(Window::default_attributes()
            .with_title("Oxide")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
            
        ).unwrap());

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested - exiting");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.clear(ColorRGBA(0, 0, 0, 255));
                self.draw_pixel(Point(WIDTH/2, HEIGHT/2), ColorRGBA(0xff, 0, 0, 0xff));
                if let Some(pixels) = self.pixels.as_mut() {
                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }, 
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    let _ = pixels.resize_surface(size.width, size.height);
                }
            },
            _ => {}
        }
    }
}

impl App {
    fn draw_pixel(&mut self, point: Point, color: ColorRGBA) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        let Point(x, y) = point;
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let idx = (y * WIDTH + x) * 4;
        frame[idx + 0] = color.0;
        frame[idx + 1] = color.1;
        frame[idx + 2] = color.2;
        frame[idx + 3] = color.3;
    }

    fn clear(&mut self, clear: ColorRGBA) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = clear.0;
            pixel[1] = clear.1;
            pixel[2] = clear.2;
            pixel[3] = clear.3;
        }
    }
}

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

struct ColorRGBA(u8, u8, u8, u8);
struct Point(usize, usize);

