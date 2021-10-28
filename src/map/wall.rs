use math_vector::Vector;
use piston_window::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wall {
    pub p1: Vector<f64>,
    pub p2: Vector<f64>,
}

impl Wall {
    pub fn new(p1: Vector<f64>, p2: Vector<f64>) -> Wall {
        Wall { p1, p2 }
    }

    pub fn get_p1(&mut self) -> Vector<f64> {
        self.p1
    }

    pub fn get_p2(&mut self) -> Vector<f64> {
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
