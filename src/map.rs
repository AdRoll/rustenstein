use crate::constants::*;
use crate::player;
use std::fmt;

#[derive(Copy, Clone)]
pub enum Tile {
    Floor,
    Wall(u16),
    Door { vertical: bool, lock: u16 },
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum Actor {
    Player(Direction),
    Enemy, // TODO differentiate enemy types
    Item,  // TODO differentiate item types
    DeadGuard,
    PushWall,
}

#[derive(Debug, Clone)]
pub struct Map {
    plane0: [[u16; MAP_HEIGHT]; MAP_WIDTH],
    plane1: [[u16; MAP_HEIGHT]; MAP_WIDTH],
    pub name: String,
}

impl Map {
    pub fn new(
        plane0: [[u16; MAP_HEIGHT]; MAP_WIDTH],
        plane1: [[u16; MAP_HEIGHT]; MAP_WIDTH],
        name: String,
    ) -> Self {
        Self {
            plane0,
            plane1,
            name,
        }
    }

    pub fn tile_at(&self, x: u8, y: u8) -> Tile {
        let tile = self.plane0[x as usize][y as usize];
        match tile {
            90 | 92 | 94 | 96 | 98 | 100 => Tile::Door {
                vertical: true,
                lock: (tile - 90) / 2,
            },
            91 | 93 | 95 | 97 | 99 | 101 => Tile::Door {
                vertical: false,
                lock: (tile - 91) / 2,
            },
            106 => Tile::Floor, // this one is actually an ambush tile, review if we need to do something with it
            n if n < 107 => Tile::Wall(tile), // keep the tile number to find the proper texture
            _ => Tile::Floor,
        }
    }

    pub fn actor_at(&self, x: u8, y: u8) -> Option<Actor> {
        match self.plane1[x as usize][y as usize] {
            19 => Some(Actor::Player(Direction::North)),
            20 => Some(Actor::Player(Direction::East)),
            21 => Some(Actor::Player(Direction::South)),
            22 => Some(Actor::Player(Direction::West)),
            n if (23..=72).contains(&n) => Some(Actor::Item),
            98 => Some(Actor::PushWall),
            124 => Some(Actor::DeadGuard),
            n if n >= 108 => Some(Actor::Enemy),
            _ => None,
        }
    }

    pub fn find_player(&self) -> player::Player {
        let (player_x, player_y, player_dir) = self.find_player_start();
        // TODO not sure why thse /3 and /2 are necessary
        let player_x = (MAP_SCALE_W * (player_x as u32) + MAP_SCALE_W / 3) as f64;
        let player_y = (MAP_SCALE_H * (player_y as u32) + MAP_SCALE_H / 2) as f64;
        let player_angle = match player_dir {
            Direction::North => ANGLE_UP,
            Direction::East => ANGLE_RIGHT,
            Direction::South => ANGLE_DOWN,
            Direction::West => ANGLE_LEFT,
        };

        player::Player {
            x: player_x,
            y: player_y,
            angle: player_angle,
        }
    }

    fn find_player_start(&self) -> (u8, u8, Direction) {
        for x in 0..MAP_WIDTH as u8 {
            for y in 0..MAP_HEIGHT as u8 {
                if let Some(Actor::Player(direction)) = self.actor_at(x, y) {
                    return (x, y, direction);
                }
            }
        }
        panic!("Can't find the player in the map");
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                let word = self.plane0[x][y];
                if word == 90 {
                    write!(f, "|").unwrap();
                } else if word == 91 {
                    write!(f, "-").unwrap();
                } else if word < 107 {
                    write!(f, "W").unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
