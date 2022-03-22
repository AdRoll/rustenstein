use crate::constants;
use crate::constants::{MAP_SCALE_H, MAP_SCALE_W};
use crate::map;
use crate::map::Tile;
use std::f64::consts::PI;

const ROTATE_SPEED: f64 = 0.2;
const MOVE_SPEED: f64 = 2.5;
const PLAYER_WIDTH: f64 = 7.0;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub view_angle: f64,
    pub move_angle: f64,
    pub is_moving_forward: bool,
    pub is_moving_backward: bool,
    pub is_moving_sideways: bool,
}

impl Player {
    pub fn turn_left(&mut self) {
        self.view_angle += ROTATE_SPEED;
        self.view_angle = constants::norm_angle(self.view_angle);
    }

    pub fn turn_right(&mut self) {
        self.view_angle -= ROTATE_SPEED;
        self.view_angle = constants::norm_angle(self.view_angle);
    }

    pub fn set_walk_forward(&mut self) {
        self.is_moving_forward = true;
        self.move_angle = self.view_angle;
    }

    pub fn set_walk_backward(&mut self) {
        self.is_moving_backward = true;
        self.move_angle = self.view_angle - PI;
    }

    pub fn set_walk_left(&mut self) {
        self.set_walk_sideways(PI / 2.0); // 90 degrees
    }

    pub fn set_walk_right(&mut self) {
        self.set_walk_sideways(-PI / 2.0); // -90 degrees
    }

    /// Moves player across the map and prevents stepping into walls.
    pub fn start_walk(&mut self, map: &map::Map) {
        if self.is_moving_sideways || self.is_moving_forward || self.is_moving_backward {
            self.walk(map);
            self.clear_walk();
        }
    }

    /// Sets walking angle for sideways move in Rads
    fn set_walk_sideways(&mut self, angle_rad: f64) {
        if self.is_moving_forward {
            self.move_angle += angle_rad / 2.0;
        } else if self.is_moving_backward {
            self.move_angle -= angle_rad / 2.0;
        } else {
            // player is moving only sideways
            self.move_angle = self.view_angle + angle_rad;
        }
    }

    fn clear_walk(&mut self) {
        self.is_moving_forward = false;
        self.is_moving_backward = false;
        self.is_moving_sideways = false;
    }

    /// Moves player across the map and prevents stepping into walls.
    /// Player collision box is a square. Its vertices are checked for collision with nearby walls.
    fn walk(&mut self, map: &map::Map) {
        // new player position (player scale)
        let new_x = self.x + self.move_angle.sin() * MOVE_SPEED;
        let new_y = self.y + self.move_angle.cos() * MOVE_SPEED;

        // new player position (map scale)
        let new_map_x = new_x / MAP_SCALE_W as f64;
        let new_map_y = new_y / MAP_SCALE_H as f64;

        // directional collision width for player (map scale)
        let collision_offset_x = self.move_angle.sin().signum() * PLAYER_WIDTH / MAP_SCALE_W as f64;
        let collision_offset_y = self.move_angle.cos().signum() * PLAYER_WIDTH / MAP_SCALE_H as f64;

        /* ■ - player angle, ╬ - checked collision vertex of player's box, o - player core position
        ■─╬
        │o│
        └─┘
        */
        let is_collision_slide_x = matches!(
            map.tile_at(
                (new_map_x + collision_offset_x) as u8,
                (new_map_y - collision_offset_y) as u8,
            ),
            Tile::Wall(_)
        );

        /* ■ - player angle, ╬ - checked collision vertex of player's box, o - player core position
        ■─┐
        │o│
        ╬─┘
        */
        let is_collision_slide_y = matches!(
            map.tile_at(
                (new_map_x - collision_offset_x) as u8,
                (new_map_y + collision_offset_y) as u8,
            ),
            Tile::Wall(_)
        );

        /* ■ - player angle and checked collision vertex of player's box, o - player core position
        ┌─■
        │o│
        └─┘
        */
        let is_collision_both = matches!(
            map.tile_at(
                (new_map_x + collision_offset_x) as u8,
                (new_map_y + collision_offset_y) as u8,
            ),
            Tile::Wall(_)
        );

        // keep moving/sliding until only both axis are colliding
        if is_collision_both && !is_collision_slide_x && !is_collision_slide_y {
            // if both axis are colliding, calculate where to slide in order to avoid clipping
            let mut whole_x = new_map_x + collision_offset_x;
            let mut whole_y = new_map_y + collision_offset_y;

            // compensate for different view angles (degrees)
            if collision_offset_x > 0.0 {
                // 270 - 0 - 90
                whole_x = whole_x.floor();
            } else {
                // 90 - 180 - 270
                whole_x = whole_x.ceil();
            }
            if collision_offset_y > 0.0 {
                // 0 - 90 - 180
                whole_y = whole_y.floor();
            } else {
                // 180 - 270 - 360
                whole_y = whole_y.ceil();
            }

            // calculate on which axis to slide
            if (new_map_x + collision_offset_x - whole_x).abs()
                > (new_map_y + collision_offset_y - whole_y).abs()
            {
                self.x = new_x;
            } else {
                self.y = new_y;
            }
        } else {
            // simple one axis slide or free movement
            if !is_collision_slide_x {
                self.x = new_x;
            }
            if !is_collision_slide_y {
                self.y = new_y;
            }
        }
    }
}
