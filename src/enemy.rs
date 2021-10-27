use piston_window::*;
use vector2d::Vector2D;

pub struct Enemy {
    pub pos: Vector2D<f64>,
    pub radius: f64,
}

fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy {
            pos: Vector2D::new(x, y),
            radius: 10.0,
        }
    }

    /// Only draw part of the enemy that is visible on the screen
    pub fn draw(
        &mut self,
        _c: &Context,
        g: &mut G2d,
        transform: math::Matrix2d,
        ppos: Vector2D<f64>,
        sight_cone: &Vec<Vector2D<f64>>,
        fov_radius: f64,
    ) {
        let (cx, cy) = (self.pos.x, self.pos.y);
        let (ax, ay) = (ppos.x, ppos.y);
        let r = self.radius;
        let f2 = fov_radius * fov_radius;

        for p2 in sight_cone {
            let (bx, by) = (p2.x, p2.y);

            let lab = f64::sqrt((bx - ax) * (bx - ax) + (by - ay) * (by - ay));
            let dx = (bx - ax) / lab;
            let dy = (by - ay) / lab;

            // line eq: x = dx*t+ax, y = dy*t+ay with 0<=t<=1
            let t = (cx - ax) * dx + (cy - ay) * dy;

            // projection of c on the line (ax, ay) (bx, by)
            let ex = ax + dx * t;
            let ey = ay + dy * t;

            let lec2 = (cx - ex) * (cx - ex) + (cy - ey) * (cy - ey);

            if lec2 >= r * r {
                continue; // no intersection
            } else if lec2 < r * r {
                let dt = f64::sqrt(r * r - lec2);

                // intersection points
                let mut fx = (t - dt) * dx + ax;
                let mut fy = (t - dt) * dy + ay;
                let mut gx = (t + dt) * dx + ax;
                let mut gy = (t + dt) * dy + ay;

                let f0 = Vector2D::new(fx, fy);
                let g0 = Vector2D::new(gx, gy);

                // behind the player
                if ((f0 - *p2).length_squared() > f2) || ((g0 - *p2).length_squared() > f2) {
                    continue;
                }

                // f0 too far
                if (f0 - ppos).length_squared() > f2 {
                    fx = p2.x;
                    fy = p2.y;
                }
                // g0 too far
                if (g0 - ppos).length_squared() > f2 {
                    gx = p2.x;
                    gy = p2.y;
                }

                line([1.0; 4], 1.0, [fx, fy, gx, gy], transform, g);
            }
        }
    }
}
