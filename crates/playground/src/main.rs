use oxide::{AnySprite, App, ColorRGBA, Game, Point, Scene, Sprite, sprites};

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
        // renderer.draw_pixel(Point(400,300), ColorRGBA(255,0,0,255));
        // renderer.draw_line(Point(0,0), Point(799,599), ColorRGBA(0, 0xff, 0, 0xff));
        // renderer.draw_rect(Point(25,25), Point(50,50), ColorRGBA(0, 0, 0xff, 0xff), true);
        // renderer.draw_rect(Point(200,200), Point(100, 100), ColorRGBA(0, 0xff, 0, 0xaa), false);
        // renderer.draw_triangle(Point(300,200), Point(200,300), Point(260, 260), ColorRGBA(0xcc, 0xcc, 0xcc, 0xcc), true);
        // renderer.draw_circle(Point(400,300), 100, ColorRGBA(0xaa, 0xbb, 0xcc, 0xdd), false);

        let mut rect = sprites::Rect::default();
        rect.init(Point(100, 100), Point(200,200), true, &renderer);
        rect.set_color(ColorRGBA(0, 0xff, 0, 0xff));
        let s1 = AnySprite::Rect(rect);

        let mut other_rect = sprites::Rect::default();
        other_rect.init(Point(150,150), Point(300, 300), false, &renderer);
        other_rect.set_color(ColorRGBA(0xff, 0, 0, 0xff));
        other_rect.set_z_index(1);
        let s2 = AnySprite::Rect(other_rect);

        let mut circle = sprites::Circle::default();
        circle.init(Point(150,150), 50, false, &renderer);
        circle.set_color(ColorRGBA(0, 0, 0xff, 0xff));
        circle.set_z_index(3);
        let s3 = AnySprite::Circle(circle);

        let mut tri = sprites::Triangle::default();
        tri.init(Point(300,300), Point(350,350), Point(320,340), true, &renderer);
        tri.set_color(ColorRGBA(0xff, 0xff, 0xff, 0xff));
        tri.set_z_index(2);
        let s4 = AnySprite::Triangle(tri);

        let mut scene = Scene::default();
        scene.push_sprite(s1);
        scene.push_sprite(s2);
        scene.push_sprite(s3);
        scene.push_sprite(s4);

        renderer.render_scene(scene);
    }
}