use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId},
};
use crate::{ColorRGBA, Point};
use crate::renderer::Renderer;
use owo_colors::OwoColorize;

pub struct App{
    window: Option<Arc<Window>>,
    renderer: Renderer,
    screen_width: usize,
    screen_height: usize,
    title: String,
}

impl App {
    pub fn new(title: impl Into<String>, width: usize, height: usize) -> Self {
        Self {
            window: None,
            renderer: Renderer::default(),
            screen_width: width, 
            screen_height: height,
            title: title.into(),
        }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.screen_width as f64, self.screen_height as f64))
            
        ).unwrap());

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels = Pixels::new(self.screen_width as u32, self.screen_height as u32, surface_texture).unwrap();

        self.window = Some(window);
        self.renderer = Renderer::new(pixels, self.screen_width, self.screen_height);

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
                println!("[{:^11}] Close requested - exiting", "INFO".green());
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                match self.renderer.render() {
                    Ok(()) => {}, 
                    Err(e) => {
                        println!("[{:^11}] {}", "Error:".bright_red().bold(), e);
                        event_loop.exit();
                    }
                };
            }, 
            WindowEvent::Resized(size) => {
                match self.renderer.resize(size) {
                    Ok(()) => {},
                    Err(e) => {
                        println!("[{:^11}] {}", "Error:".bright_red().bold(), e);
                        event_loop.exit();
                    }
                };
            },
            _ => {}
        }
    }
}