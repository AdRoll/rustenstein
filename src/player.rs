use crate::constants;
use crate::constants::{MAP_SCALE_H, MAP_SCALE_W};
use crate::map;
use crate::map::Tile;
use std::f64::consts::PI;

const ROTATE_SPEED: f64 = 0.2;
const MOVE_SPEED: f64 = 2.5;
const PLAYER_WIDTH: f64 = 7.0;

pub enum StraightMovement {
    Forward,
    Backward,
}

pub enum SideMovement {
    StrafeRight,
    StrafeLeft,
}

pub enum TurnMovement {
    TurnRight,
    TurnLeft,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub view_angle: f64,
    pub move_angle: f64,
}

impl Player {
    /// Moves player across the map and prevents stepping into walls.
    /// Player collision box is a square. Its vertices are checked for collision with nearby walls.
    pub fn walk(
        &mut self,
        map: &map::Map,
        straight: Option<StraightMovement>,
        side: Option<SideMovement>,
        turn: Option<TurnMovement>,
        run: bool,
    ) {
        self.view_angle = match turn {
            Some(TurnMovement::TurnLeft) => constants::norm_angle(self.view_angle + ROTATE_SPEED),
            Some(TurnMovement::TurnRight) => constants::norm_angle(self.view_angle - ROTATE_SPEED),
            None => self.view_angle,
        };

        if side.is_some() || straight.is_some() {
            self.move_angle = match straight {
                Some(StraightMovement::Forward) => match side {
                    Some(SideMovement::StrafeLeft) => self.view_angle + PI / 4.0,
                    Some(SideMovement::StrafeRight) => self.view_angle - PI / 4.0,
                    None => self.view_angle,
                },
                Some(StraightMovement::Backward) => match side {
                    Some(SideMovement::StrafeLeft) => self.view_angle - PI - PI / 4.0,
                    Some(SideMovement::StrafeRight) => self.view_angle - PI + PI / 4.0,
                    None => self.view_angle - PI,
                },
                None => match side {
                    Some(SideMovement::StrafeLeft) => self.view_angle + PI / 2.0,
                    Some(SideMovement::StrafeRight) => self.view_angle - PI / 2.0,
                    None => self.move_angle, // will never happen
                },
            };

            let current_move_speed = match run {
                true => MOVE_SPEED * 2.0,
                false => MOVE_SPEED * 2.0,
            };

            // new player position (player scale)
            let new_x = self.x + self.move_angle.sin() * current_move_speed;
            let new_y = self.y + self.move_angle.cos() * current_move_speed;

            // new player position (map scale)
            let new_map_x = new_x / MAP_SCALE_W as f64;
            let new_map_y = new_y / MAP_SCALE_H as f64;

            // directional collision width for player (map scale)
            let collision_offset_x =
                self.move_angle.sin().signum() * PLAYER_WIDTH / MAP_SCALE_W as f64;
            let collision_offset_y =
                self.move_angle.cos().signum() * PLAYER_WIDTH / MAP_SCALE_H as f64;

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
}
