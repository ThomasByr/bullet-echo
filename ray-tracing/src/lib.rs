use math_vector::Vector;

pub struct Ray {
    pub origin: Vector<f64>,
    pub dir: Vector<f64>,
}

pub struct Wall {
    pub p1: Vector<f64>,
    pub p2: Vector<f64>,
}

impl Wall {
    pub fn new(p1: Vector<f64>, p2: Vector<f64>) -> Wall {
        Wall { p1, p2 }
    }
}

impl Ray {
    pub fn new(origin: Vector<f64>, dir: Vector<f64>) -> Ray {
        Ray { origin, dir }
    }

    pub fn point_at(&mut self, p: Vector<f64>) {
        let dir = p - self.origin;
        self.dir = dir.normalise();
    }

    pub fn cast(&mut self, wall: &Wall) -> Option<Vector<f64>> {
        let x1 = wall.p1.x;
        let y1 = wall.p1.y;
        let x2 = wall.p2.x;
        let y2 = wall.p2.y;
        let x3 = self.origin.x;
        let y3 = self.origin.y;
        let x4 = self.origin.x + self.dir.x;
        let y4 = self.origin.y + self.dir.y;

        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / d;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 {
            let mut p = Vector::default();
            p.x = x1 + t * (x2 - x1);
            p.y = y1 + t * (y2 - y1);
            return Some(p);
        }
        None
    }

    pub fn look(&mut self, walls: &Vec<Wall>) -> Option<Vector<f64>> {
        let mut record = std::f64::MAX;
        let mut closest = None;
        for wall in walls {
            if let Some(pt) = self.cast(wall) {
                let d = self.origin.distance(pt);
                if d < record {
                    record = d;
                    closest = Some(pt);
                }
            }
        }
        closest
    }
}
