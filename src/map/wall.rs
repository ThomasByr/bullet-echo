use piston_window::*;
use vector2d::Vector2D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wall {
    pub p1: Vector2D<f64>,
    pub p2: Vector2D<f64>,
}

impl Wall {
    pub fn new(p1: Vector2D<f64>, p2: Vector2D<f64>) -> Wall {
        Wall { p1, p2 }
    }

    pub fn get_p1(&mut self) -> Vector2D<f64> {
        self.p1
    }

    pub fn get_p2(&mut self) -> Vector2D<f64> {
        self.p2
    }

    pub fn draw(&mut self, _c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        line(
            [0.0, 0.0, 0.0, 1.0],
            1.0,
            [self.p1.x, self.p1.y, self.p2.x, self.p2.y],
            transform,
            g,
        );
    }
}
