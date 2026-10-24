//! Renderer functions for drawing geometric shapes

use std::cmp::{min, max};
use crate::{ColorRGBA, Renderer, Point};

impl Renderer {
    /// Draw a rectangle given the endpoints of its diagonal, 
    /// using specified color and fill option.
    pub fn draw_rect(&mut self, p1: Point, p2: Point, color: ColorRGBA, fill: bool) {
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        if x1 >= self.screen_width || y1 >= self.screen_height || x2 >= self.screen_width || y2 >= self.screen_height { return; }

        let origin = Point(min(x1, x2), min(y1, y2));
        let dest = Point(max(x1, x2), max(y1, y2));

        if fill {
            for x in origin.0 .. dest.0 {
                for y in origin.1 .. dest.1 {
                    self.draw_pixel(Point(x,y), color);
                }
            }
        } else {
            self.draw_line(Point(x1,y1), Point(x2,y1), color);
            self.draw_line(Point(x1,y1), Point(x1,y2), color);
            self.draw_line(Point(x1,y2), Point(x2,y2), color);
            self.draw_line(Point(x2,y1), Point(x2,y2), color);
        }
    }

    /// Draw a triangle given its vertices, 
    /// using specified color and fill option.
    pub fn draw_triangle(&mut self, p1: Point, p2: Point, p3: Point, color: ColorRGBA, fill: bool) {
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        let Point(x3, y3) = p3;
        if x1 >= self.screen_width || y1 >= self.screen_height || x2 >= self.screen_width || y2 >= self.screen_height || x3 >= self.screen_width || y3 >= self.screen_height { return; }

        if fill {
            let mut vertices = [(x1,y1), (x2,y2), (x3,y3)];
            vertices.sort_by_key(|v| v.1);
            let [(x1,y1), (x2,y2), (x3,y3)] = vertices;

            let interpolate_x = |y: usize, ya: usize, xa: usize, yb: usize, xb: usize| -> usize {
                if yb == ya {
                    return xa;
                }
                let ya = ya as i32;
                let yb = yb as i32;
                let xa = xa as i32;
                let xb = xb as i32;
                let y = y as i32;
                
                (xa + (xb - xa) * (y - ya) / (yb - ya)) as usize
            };

            // Fill the triangle by splitting it into two parts
            // Top part: from y1 to y2
            for y in y1..=y2 {
                let x_left = interpolate_x(y, y1, x1, y3, x3);
                let x_right = interpolate_x(y, y1, x1, y2, x2);
                
                let (x_start, x_end) = if x_left <= x_right {
                    (x_left, x_right)
                } else {
                    (x_right, x_left)
                };
                
                for x in x_start..=x_end {
                    self.draw_pixel(Point(x,y), color);
                }
            }

            // Bottom part: from y2 to y3
            for y in (y2 + 1)..=y3 {
                let x_left = interpolate_x(y, y1, x1, y3, x3);
                let x_right = interpolate_x(y, y2, x2, y3, x3);
                
                let (x_start, x_end) = if x_left <= x_right {
                    (x_left, x_right)
                } else {
                    (x_right, x_left)
                };
                
                for x in x_start..=x_end {
                    self.draw_pixel(Point(x,y), color);
                }
            }

        } else {
            self.draw_line(p1, p2, color);
            self.draw_line(p3, p2, color);
            self.draw_line(p1, p3, color);
        }
    }
}