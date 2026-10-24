use oxide::{App, ColorRGBA, Game, Point};

fn main() {
    println!("Hello, world!");
    let app = App::new("Oxide Playground", 800, 600);
    app.run(Playground {}).expect("Event loop error.");
}

struct Playground {}

impl Game for Playground {
    fn update(&mut self, _dt: f32) {
        
    }
    fn draw(&mut self, renderer: &mut oxide::Renderer) {
        renderer.clear(ColorRGBA(0,0,0,255));
        renderer.draw_pixel(Point(400,300), ColorRGBA(255,0,0,255));
        renderer.draw_line(Point(0,0), Point(799,599), ColorRGBA(0, 0xff, 0, 0xff));
        renderer.draw_rect(Point(25,25), Point(50,50), ColorRGBA(0, 0, 0xff, 0xff), true);
        renderer.draw_rect(Point(200,200), Point(100, 100), ColorRGBA(0, 0xff, 0, 0xaa), false);
        renderer.draw_triangle(Point(300,200), Point(200,300), Point(260, 260), ColorRGBA(0xcc, 0xcc, 0xcc, 0xcc), true);
        renderer.draw_circle(Point(400,300), 100, ColorRGBA(0xaa, 0xbb, 0xcc, 0xdd), false);
    }
}