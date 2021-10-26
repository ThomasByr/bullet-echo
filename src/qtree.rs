use vector2d::Vector2D;

pub mod bounds;
use bounds::*;
use piston_window::*;

pub struct QTree {
    pub northeast: Option<Box<QTree>>,
    pub northwest: Option<Box<QTree>>,
    pub southeast: Option<Box<QTree>>,
    pub southwest: Option<Box<QTree>>,

    pub center: Vector2D<f64>,
    pub size: usize,
    pub points: Vec<Point>,

    pub boundary: Rect,
    pub is_divided: bool,
}

impl QTree {
    pub fn new(center: Vector2D<f64>, size: usize, width: f64, height: f64) -> QTree {
        QTree {
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
            center,
            size,
            points: Vec::new(),
            boundary: Rect {
                pos: center.clone(),
                width,
                height,
            },
            is_divided: false,
        }
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        self.boundary.draw(c, g, transform);
        if self.is_divided {
            self.northeast.as_mut().unwrap().draw(c, g, transform);
            self.northwest.as_mut().unwrap().draw(c, g, transform);
            self.southeast.as_mut().unwrap().draw(c, g, transform);
            self.southwest.as_mut().unwrap().draw(c, g, transform);
        }
    }

    pub fn subdivide(&mut self) {
        self.is_divided = true;
        let x = self.boundary.pos.x;
        let y = self.boundary.pos.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        self.northeast = Some(Box::new(QTree::new(
            Vector2D::new(x - w, y - h),
            self.size,
            w,
            h,
        )));
        self.northwest = Some(Box::new(QTree::new(
            Vector2D::new(x + w, y - h),
            self.size,
            w,
            h,
        )));
        self.southeast = Some(Box::new(QTree::new(
            Vector2D::new(x - w, y + h),
            self.size,
            w,
            h,
        )));
        self.southwest = Some(Box::new(QTree::new(
            Vector2D::new(x + w, y + h),
            self.size,
            w,
            h,
        )));
    }

    pub fn insert(&mut self, p: &Point) -> bool {
        if !(self.boundary.contains(p)) {
            false
        } else if self.points.len() < self.size {
            self.points.push(*p);
            true
        } else {
            if !self.is_divided {
                self.subdivide();
            }
            if self.northeast.as_mut().unwrap().insert(p) {
                true
            } else if self.northwest.as_mut().unwrap().insert(p) {
                true
            } else if self.southeast.as_mut().unwrap().insert(p) {
                true
            } else if self.southwest.as_mut().unwrap().insert(p) {
                true
            } else {
                false
            }
        }
    }

    pub fn query(&mut self, rect: &Rect) -> Vec<Point> {
        let mut points = Vec::new();
        if !(self.boundary.intersects_r(rect)) {
            return points;
        }
        for p in &self.points {
            if rect.contains(p) {
                points.push(*p);
            }
        }
        if self.is_divided {
            points.append(&mut self.northeast.as_mut().unwrap().query(rect));
            points.append(&mut self.northwest.as_mut().unwrap().query(rect));
            points.append(&mut self.southeast.as_mut().unwrap().query(rect));
            points.append(&mut self.southwest.as_mut().unwrap().query(rect));
        }
        points
    }

    pub fn clear(&mut self) {
        self.points.clear();
        if self.is_divided {
            self.northeast.as_mut().unwrap().clear();
            self.northwest.as_mut().unwrap().clear();
            self.southeast.as_mut().unwrap().clear();
            self.southwest.as_mut().unwrap().clear();
        }
        self.is_divided = false;
    }
}
