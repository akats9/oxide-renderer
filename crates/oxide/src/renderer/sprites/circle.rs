//! Circle Sprite object

use crate::{ColorRGBA, Point, Renderer, Sprite};
use super::{Velocity, Shape, Box};

/// Circle Sprite type.
pub struct Circle {
    origin: Point,
    radius: usize,
    v: Velocity,
    color: ColorRGBA,
    shown: bool,
    z: usize,
    fill: bool,
}

impl Sprite for Circle {
    fn position(&self) -> Point {
        self.origin
    }

    fn velocity(&self) -> Velocity {
        self.v
    }

    fn shape(&self) -> Shape {
        Shape::Circle
    }

    fn color(&self) -> ColorRGBA {
        self.color
    }

    fn z_index(&self) -> usize {
        self.z
    }

    fn shown(&self) -> bool {
        self.shown
    }

    fn set_position(&mut self, p: Point) {
        self.origin = p;
    }

    fn set_velocity(&mut self, v: Velocity) {
        self.v = v;
    }

    fn set_color(&mut self, c: ColorRGBA) {
        self.color = c;
    }

    fn set_z_index(&mut self, z: usize) {
        self.z = z;
    }

    fn set_shown(&mut self, s: bool) {
        self.shown = s;
    }

    fn clear(&mut self, r: &mut Renderer, bg: ColorRGBA) -> Box {
        unimplemented!()
    }

    fn is_in_bounding_box(&self, b: Box) -> bool {
        let hitbox = Box(Point(self.origin.0 - self.radius, self.origin.1 - self.radius), Point(self.origin.0 + self.radius, self.origin.1 + self.radius));
        if b.contains_box(hitbox) { return true; }
        false
    }

    fn draw(&mut self, r: &mut Renderer) {
        r.draw_circle(self.origin, self.radius, self.color, self.fill);
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            origin: Point(0,0),
            radius: 0,
            v: Velocity(0, 0),
            color: ColorRGBA(0, 0, 0, 0),
            shown: false,
            z: 0,
            fill: true,
        }
    }
}

impl Circle {
    /// Initializes the dimensions and starting position
    /// of the Sprite and sets `shown` to `true`.
    pub fn init(&mut self, origin: Point, radius: usize, fill: bool, r: &Renderer) {
        let screen_bounds = Box(Point(0,0), Point(r.screen_width, r.screen_height));
        let hitbox = Box(Point(origin.0.saturating_sub(radius), origin.1.saturating_sub(radius)), Point(origin.0 + radius, origin.1 + radius));
        if !screen_bounds.contains_box(hitbox) { return; }

        self.origin = origin;
        self.radius = radius;
        self.fill = fill;
        self.set_shown(true);
    }
}