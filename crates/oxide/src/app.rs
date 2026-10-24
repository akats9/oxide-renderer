//! App module.
//! Provides methods to create and initialize a
//! winit window, GPU surface and event loop.

use std::sync::Arc;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId},
};
use crate::Game;
use crate::renderer::Renderer;
use owo_colors::OwoColorize;

/// Represents the game window.
/// Holds Renderer and Game as well as params.
pub struct App<G: Game> {
    window: Option<Arc<Window>>,
    renderer: Renderer,
    screen_width: usize,
    screen_height: usize,
    title: String,
    game: Option<G>,
}

impl<G: Game> App<G> {
    pub fn new(title: impl Into<String>, width: usize, height: usize) -> Self {
        Self {
            window: None,
            renderer: Renderer::default(),
            screen_width: width, 
            screen_height: height,
            title: title.into(),
            game: None,
        }
    }

    pub fn run(mut self, game: G) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new()?;
        self.game = Some(game);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<G: Game> ApplicationHandler for App<G> {
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
                match self.redraw() {
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

impl<G: Game> App<G> {
    fn redraw(&mut self) -> Result<(), Error> {
        let dt = 1.0 / 60.0;

        if let Some(game) = self.game.as_mut() {
            game.update(dt);
            game.draw(&mut self.renderer);
        }

        self.renderer.render()?;

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }

        Ok(())
    }
}