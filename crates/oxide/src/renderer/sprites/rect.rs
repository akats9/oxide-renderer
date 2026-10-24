//! Rectangle Sprite object.

use crate::{ColorRGBA, Point, Sprite};
use super::{Velocity, Shape};

pub struct Rect {
    pos: Point,
    p1: Point,
    p2: Point, 
    v: Velocity,
    color: ColorRGBA,
    shown: bool,
    z: usize,
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

    fn clear(&mut self) {
        unimplemented!()
    }

    fn draw(&mut self) {
        unimplemented!()
    }

    fn is_in_bounding_box(&self, p1: Point, p2: Point) -> bool {
        unimplemented!()
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
            shown: true,
            z: 0,
        }
    }
}