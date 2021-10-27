use piston_window::*;

use vector2d::Vector2D;

pub mod element;
pub mod wall;

use element::Element;
use wall::Wall;

use crate::{qtree::QTree, HEIGHT, WIDTH};

pub struct Map {
    pub elements: Vec<Element>, // map elements (textures, loots, ...)
    pub walls: Vec<Wall>,       // all walls
    pub sqtree: QTree,          // static quadtree
    pub dqtree: QTree,          // dynamic quadtree
}

impl Map {
    pub fn new() -> Map {
        let (w, h) = (WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
        let center = Vector2D::new(0.0, 0.0);
        Map {
            elements: Vec::new(),
            walls: Vec::new(),
            sqtree: QTree::new(center, 4, w, h),
            dqtree: QTree::new(center, 4, w, h),
        }
    }

    /// Draw the quadtrees (useful for debugging)
    pub fn draw_qt(&mut self, c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        self.sqtree.draw(c, g, transform);
        self.dqtree.draw(c, g, transform);
    }

    /// Draw the map (walls and elements)
    pub fn draw(&mut self, c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        for wall in self.walls.iter_mut() {
            wall.draw(c, g, transform);
        }
    }
}
