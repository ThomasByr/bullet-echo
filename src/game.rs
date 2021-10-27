use piston_window::*;
// use rand::Rng;
use serde_json::{from_str, Value};
use vector2d::Vector2D;

use super::{enemy::*, map::wall::Wall, map::*, player::*};
use crate::{HEIGHT, PI, WIDTH};

pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub map: Map,
    pub key_pressed: Vec<Key>,

    pub score: u32,
    pub level: u32,

    pub game_over: bool,
    pub rng: rand::rngs::ThreadRng,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
            enemies: Vec::new(),
            map: Map::new(),
            key_pressed: Vec::new(),

            score: 0,
            level: 1,

            game_over: false,
            rng: rand::thread_rng(),
        }
    }

    pub fn spawn_enemy(&mut self) {
        let enemy = Enemy::new(300f64, -300f64);
        self.enemies.push(enemy);
    }

    pub fn add_key_pressed(&mut self, key: Key) {
        if !self.key_pressed.contains(&key) {
            self.key_pressed.push(key);
        }
    }

    pub fn remove_key_pressed(&mut self, key: Key) {
        if self.key_pressed.contains(&key) {
            self.key_pressed.retain(|&x| x != key);
        }
    }

    pub fn load_map(&mut self, path: &str) {
        self.map = Map::new();
        let data = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not read file {}", path));
        let v: Value = from_str(&data).unwrap_or_else(|_| panic!("Could not parse file {}", path));

        let walls = v["walls"].as_array().unwrap().to_vec();

        for w in walls {
            let p1 = Vector2D::new(w["p1"][0].as_f64().unwrap(), w["p1"][1].as_f64().unwrap());
            let p2 = Vector2D::new(w["p2"][0].as_f64().unwrap(), w["p2"][1].as_f64().unwrap());
            let wall = Wall::new(p1, p2);
            self.map.walls.push(wall);
        }
    }

    pub fn update(&mut self) {
        // update player position and heading based on key pressed
        for key in self.key_pressed.iter() {
            match key {
                Key::Up => self.player.move_player(1.0),
                Key::Z => self.player.move_player(1.0),
                Key::Down => self.player.move_player(-1.0),
                Key::S => self.player.move_player(-1.0),
                Key::Left => self.player.turn_player(-1.0),
                Key::Right => self.player.turn_player(1.0),
                Key::Q => self.player.slide_player(-1.0),
                Key::D => self.player.slide_player(1.0),
                _ => (),
            }
        }
        self.player.update_player();
        self.player.compute_view(&self.map.walls)
    }

    pub fn draw(&mut self, c: &Context, g: &mut G2d, _glyphs: &mut Glyphs) {
        let (cx, cy) = (WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
        let (x0, y0) = (self.player.pos.x, self.player.pos.y);

        let mut transform = c.transform.trans(cx, cy); // translate to center
        transform = transform.rot_rad(-self.player.heading.angle() - PI / 2.0); // rotate
        transform = transform.trans(-x0, -y0); // translate to player

        self.map.draw(c, g, transform); // draw map
        self.player.draw(c, g, transform); // draw player

        for e in self.enemies.iter_mut() {
            e.draw(
                c,
                g,
                transform,
                self.player.pos,
                &self.player.sight_cone,
                self.player.fov_radius,
            );
        }

        // self.map.draw_qt(c, g, transform);
    }
}
