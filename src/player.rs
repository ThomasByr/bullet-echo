use piston_window::*;
use vector2d::Vector2D;

use ray_tracing::{Ray as RayTracingRay, Wall as RayTracingWall};

use crate::{map::wall::Wall, PI, RAYS};

pub struct Player {
    pub max_health: f64,  // maximum health of player
    pub health: f64,      // current health of player
    pub ammo: u64,        // current ammo of player
    pub in_mag: u64,      // current ammo in magazine
    pub fire_rate: u64,   // fire rate of player (number of frames between shots)
    pub reload_time: u64, // reload time of player (number of frames between reloading)

    pub pos: Vector2D<f64>, // position of player
    pub vel: Vector2D<f64>, // velocity of player
    pub rot: f64,           // rotation of player
    pub lin_vel: f64,       // linear velocity of player
    pub rot_vel: f64,       // angular velocity of player
    pub radius: f64,        // size of player (for drawing)

    pub heading: Vector2D<f64>, // heading of player
    pub accuracy: f64,          // accuracy (cone of fire) of player (red)
    pub accuracy_rec: f64,      // accuracy decrease / increase when moving
    pub min_acc: f64,           // minimum accuracy
    pub max_acc: f64,           // maximum accuracy
    pub fov: f64,               // field of view (grey)
    pub fov_radius: f64,        // max radius of field of view (grey)

    pub health_bar_length: f64,        // length of health bar
    pub health_bar_height: f64,        // height of health bar
    pub health_bar_pos: Vector2D<f64>, // position of health bar (relative to center of player)

    pub sight_cone: Vec<Vector2D<f64>>, // points of sight cone
    pub color_cone: Vec<[f32; 4]>,      // colors of rays in sight cone
    pub hit_cone: Vec<bool>,            // whether ray hit wall in sight cone
}

impl Player {
    pub fn new() -> Player {
        Player {
            max_health: 100.0,
            health: 100.0,
            ammo: 0,
            in_mag: 0,
            fire_rate: 30,
            reload_time: 600,

            pos: Vector2D::new(0.0, 0.0),
            vel: Vector2D::new(0.0, 0.0),
            rot: 0.0,
            lin_vel: 1.0,
            rot_vel: 2.0 * PI / 360.0,
            radius: 10.0,

            heading: Vector2D::new(0.0, -1f64),
            accuracy: PI / 32.0,
            accuracy_rec: 5e-4,
            min_acc: PI / 64.0,
            max_acc: PI / 4.0,
            fov: PI / 4.0,
            fov_radius: 200.0,

            health_bar_length: 30.0,
            health_bar_height: 3.0,
            health_bar_pos: Vector2D::new(-15.0, -15.0),

            sight_cone: Vec::new(),
            color_cone: Vec::new(),
            hit_cone: Vec::new(),
        }
    }

    pub fn decrease_accuracy(&mut self) {
        self.accuracy += self.accuracy_rec;
        if self.accuracy > self.max_acc {
            self.accuracy = self.max_acc;
        }
    }

    pub fn increase_accuracy(&mut self) {
        self.accuracy -= self.accuracy_rec;
        if self.accuracy < self.min_acc {
            self.accuracy = self.min_acc;
        }
    }

    pub fn stop_player(&mut self) {
        self.vel *= 0.0;
        self.rot = 0.0;
    }

    pub fn update_player(&mut self) {
        self.pos += self.vel;
        self.heading = self.heading.rotate(self.rot);

        if self.vel.length_squared() < 1e-8 && f64::abs(self.rot) < 1e-8 {
            self.increase_accuracy();
        } else {
            self.decrease_accuracy();
        }

        self.stop_player();
    }

    pub fn move_player(&mut self, dir: f64) {
        self.vel += self.heading.normalise() * dir * self.lin_vel;
    }

    pub fn slide_player(&mut self, dir: f64) {
        self.vel += self.heading.rotate(PI / 2.0).normalise() * dir * self.lin_vel;
    }

    pub fn turn_player(&mut self, dir: f64) {
        self.rot = dir * self.rot_vel;
    }

    /// Calculates the points of sight cone of the player
    pub fn compute_view(&mut self, walls: &Vec<Wall>) {
        self.sight_cone.clear();
        self.color_cone.clear();
        self.hit_cone.clear();

        let heading = self.heading;
        let fov = self.fov;

        for i in 0..RAYS {
            let if64 = i as f64;
            let angle = if64 * fov / RAYS as f64 - fov / 2.0;
            let dir = heading.rotate(angle);

            let col = if f64::abs(angle) < self.accuracy * 2.0 {
                [0.3, 0.2, 0.2, 1.0]
            } else {
                [0.3, 0.3, 0.3, 1.0]
            };

            let mut ray = RayTracingRay::new(self.pos, dir);
            let rtwalls = walls
                .iter()
                .map(|w| RayTracingWall::new(w.p1, w.p2))
                .collect::<Vec<RayTracingWall>>();

            let pt = ray.look(&rtwalls);
            let mut hit = false;
            let mut p = match pt {
                Some(p) => {
                    hit = true;
                    p
                }
                None => self.pos + dir * self.fov_radius,
            };

            let d = self.pos.distance(p);
            if d > self.fov_radius {
                p = self.pos + dir * self.fov_radius;
            }
            let px = p.x - self.pos.x;
            let py = p.y - self.pos.y;

            self.sight_cone.push(Vector2D::new(px, py));
            self.color_cone.push(col);
            self.hit_cone.push(hit && d <= self.fov_radius);
        }
    }

    /// Draws the player
    pub fn draw(&mut self, _c: &Context, g: &mut G2d, transform: math::Matrix2d) {
        let heading = self.heading;
        let radius = self.radius;

        // draw player as a circle
        ellipse(
            [0.0, 1.0, 0.0, 1.0],
            [
                -self.radius,
                -self.radius,
                self.radius * 2.0,
                self.radius * 2.0,
            ],
            transform,
            g,
        );

        // draw rays
        for i in 0..RAYS {
            let (px, py) = (self.sight_cone[i].x, self.sight_cone[i].y);
            let col = self.color_cone[i];
            let hit = self.hit_cone[i];

            line(col, 1.0, [0.0, 0.0, px, py], transform, g);
            if hit {
                line([1.0; 4], 1.0, [px, py - 1.0, px, py], transform, g);
            }
        }

        // render direction
        line(
            [1.0; 4],
            1.0,
            [0.0, 0.0, heading.x * radius, heading.y * radius],
            transform,
            g,
        );
    }
}
