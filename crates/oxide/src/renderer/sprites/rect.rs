//! Rectangle Sprite object.

use std::cmp::{max, min};

use crate::{ColorRGBA, Point, Renderer, Sprite};
use super::{Velocity, Shape, Box};

/// Rectangle Sprite type. 
/// `p1` and `p2` are the endpoints of its diagonal,
/// `pos` is the geometric centroid.
pub struct Rect {
    pos: Point,
    p1: Point,
    p2: Point, 
    v: Velocity,
    color: ColorRGBA,
    shown: bool,
    z: usize,
    fill: bool,
}

impl Sprite for Rect {
    fn position(&self) -> Point {
        self.pos
    }

    fn velocity(&self) -> Velocity {
        self.v
    }

    fn shape(&self) -> Shape {
        Shape::Rect
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
        r.draw_rect(self.p1, self.p2, bg, self.fill);

        Box(self.p1, self.p2)
    }

    fn draw(&mut self, r: &mut Renderer) {
        r.draw_rect(self.p1, self.p2, self.color, self.fill);
    }

    fn is_in_bounding_box(&self, b: Box) -> bool {
        let Point(x1,y1) = self.p1;
        let Point(x2, y2) = self.p2;
        let Point(x3, y3) = Point(x1, y2);
        let Point(x4, y4) = Point(x2, y1);

        let Box(b1, b2) = b;
        let Point(bx1, by1) = b1;
        let Point(bx2, by2) = b2;

        if 
            is_between(x1, bx1, bx2) && is_between(y1, by1, by2) 
            || is_between(x2, bx1, bx2) && is_between(y2, by1, by2)
            || is_between(x3, bx1, bx2) && is_between(y3, by1, by2)
            || is_between(x4, bx1, bx2) && is_between(y4, by1, by2)
        {
            return true;
        }

        false
    }

    fn animate(&mut self, end_pos: Point, duration: usize) {
        unimplemented!()
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            pos: Point(0, 0),
            p1: Point(0, 0),
            p2: Point(1,1),
            v: Velocity(0, 0),
            color: ColorRGBA(0xff, 0xff, 0xff, 0xff),
            shown: false,
            z: 0,
            fill: true,
        }
    }
}

impl Rect {
    /// Initializes the dimensions and starting position 
    /// of the Sprite and sets `shown` to `true`.
    fn init(&mut self, p1: Point, p2: Point, fill: bool) {
        self.p1 = p1;
        self.p2 = p2;
        self.fill = fill;
        self.pos = {
            let Point(x1, y1) = p1;
            let Point(x2, y2) = p2;
            Point((x1+x2)/2, (y1+y2)/2)
        };
        self.set_shown(true);
    }
}

fn is_between(v: usize, a: usize, b: usize) -> bool {
    let min = min(a, b);
    let max = max(a, b);
    (min..=max).contains(&v)
}