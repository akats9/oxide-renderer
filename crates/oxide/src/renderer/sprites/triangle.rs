//! Triangle Sprite object.

use min_max::*;
use crate::{ColorRGBA, Point, Renderer, Sprite};
use super::{Velocity, Shape, Box};

pub struct Triangle {
    pos: Point,
    p1: Point, 
    p2: Point, 
    p3: Point,
    v: Velocity,
    color: ColorRGBA,
    shown: bool,
    z: usize,
    fill: bool,
}

impl Sprite for Triangle {
    fn position(&self) -> Point {
        self.pos
    }

    fn velocity(&self) -> Velocity {
        self.v
    }

    fn shape(&self) -> Shape {
        Shape::Triangle
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
        self.pos = p;
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
        r.draw_triangle(self.p1, self.p2, self.p3, bg, self.fill);
        let Point(x1,y1) = self.p1;
        let Point(x2, y2) = self.p2;
        let Point(x3, y3) = self.p3;
        Box(
            Point(min_max::min!(x1,x2,x3), min_max::min!(y1,y2,y3)),
            Point(min_max::max!(x1,x2,x3), min_max::max!(y1,y2,y3))
        )
    }

    fn draw(&mut self, r: &mut Renderer) {
        r.draw_triangle(self.p1, self.p2, self.p3, self.color, self.fill);
    }

    fn is_in_bounding_box(&self, b: Box) -> bool {
        if b.contains(self.p1) || b.contains(self.p2) || b.contains(self.p3) { return true; }
        false
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            pos: Point(0, 0),
            p1: Point(0, 0),
            p2: Point(0, 0),
            p3: Point(0, 0),
            v: Velocity(0, 0),
            color: ColorRGBA(0, 0, 0, 0),
            shown: false,
            z: 0,
            fill: true
        }
    }
}

impl Triangle {
    /// Initializes the dimensions and starting position
    /// of the Sprite and sets `shown` to `true`.
    pub fn init(&mut self, p1: Point, p2: Point, p3: Point, fill: bool, r: &Renderer) {
        let screen_bounds = Box(Point(0,0), Point(r.screen_width, r.screen_height));
        if !screen_bounds.contains(p1) || !screen_bounds.contains(p2) || !screen_bounds.contains(p3) { return; }

        self.p1 = p1;
        self.p2 = p2;
        self.p3 = p3;

        self.pos = {
            let Point(x1,y1) = p1;
            let Point(x2, y2) = p2;
            let Point(x3,y3) = p3;
            Point((x1+x2+x3)/3, (y1+y2+y3)/3)
        };
        self.fill = fill;
        self.shown = true;
    }
}