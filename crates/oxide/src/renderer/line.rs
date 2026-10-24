//! Renderer functions for drawing lines.

use crate::{ColorRGBA, Point, Renderer};

impl Renderer {
    /// Draws a line between two points with specified color using Bresenham's algorithm
    pub fn draw_line(&mut self, p1: Point, p2: Point, color: ColorRGBA) {
        let Point(mut x, mut y) = p1;
        let Point(x2, y2) = p2;

        if x >= self.screen_width || y >= self.screen_height || x2 >= self.screen_width || y2 >= self.screen_height { return; }

        let dx = x.abs_diff(x2) as isize;
        let dy = y.abs_diff(y2) as isize;
        let sx: isize = if x < x2 { 1 } else { -1 };
        let sy: isize = if y < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.draw_pixel(Point(x,y), color);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= -dy {
                err -= dy;
                x = x.saturating_add_signed(sx);
            }
            if e2 <= dx {
                err += dx; 
                y = y.saturating_add_signed(sy);
            }
        }
    }
}