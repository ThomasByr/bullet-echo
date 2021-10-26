use piston_window::*;
use vector2d::Vector2D;

pub struct Enemy {
    pub pos: Vector2D<f64>,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy {
            pos: Vector2D::new(x, y),
        }
    }

    pub fn draw(&mut self, _c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        let square = rectangle::square(self.pos.x, self.pos.y, 10.0);
        rectangle([1.0, 0.0, 0.0, 1.0], square, transform, g);
    }
}
