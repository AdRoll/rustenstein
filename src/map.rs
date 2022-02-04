use std::{fmt, fs};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 64;

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

#[derive(Debug)]
pub struct Map {
    plane0: [[u16; HEIGHT]; WIDTH],
    plane1: [[u16; HEIGHT]; WIDTH],
    pub name: String,
}

impl Map {
    pub fn new(
        plane0: [[u16; HEIGHT]; WIDTH],
        plane1: [[u16; HEIGHT]; WIDTH],
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

    pub fn find_player_start(&self) -> (u8, u8, Direction) {
        for x in 0..WIDTH as u8 {
            for y in 0..HEIGHT as u8 {
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
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
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
