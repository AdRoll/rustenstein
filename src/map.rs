use std::path::Path;
use std::{fmt, fs};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 64;

// see some map plans here: https://wolfenstein.fandom.com/wiki/Wolfenstein_3D
// some map format info: https://moddingwiki.shikadi.net/wiki/GameMaps_Format
// on the RLEW compression algorithm: https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression

// first goal is to get the job done, we'll care about efficiency later and even then,
// the parsing is so small that it should not really matter for our purposes anyways

#[derive(Debug)]
struct MapHead {
    magic: [u8; 2],
    pointers: Vec<i32>,
    title: Vec<u8>, // so far empty in map files we've seen, no idea what to do with this for now... could probably be a string
}

fn parse_map_head<P: AsRef<Path>>(path: P, keep_n_first: Option<usize>) -> MapHead {
    let raw_data = fs::read(path).expect("could not read MAPHEAD file");

    MapHead {
        magic: raw_data[0..2].try_into().unwrap(),
        pointers: raw_data[2..=(4 * 100)]
            .chunks_exact(4)
            .take(keep_n_first.unwrap_or(100))
            .map(|x| i32::from_le_bytes(x.try_into().unwrap()))
            .filter(|&x| x > 0)
            .collect(),
        title: raw_data[(2 + (4 * 100))..].to_owned(),
    }
}

#[derive(Debug)]
struct MapLevelHeader {
    offset_plane0: i32,
    offset_plane1: i32,
    offset_plane2: i32,
    length_plane0: u16,
    length_plane1: u16,
    length_plane2: u16,
    width_n_tiles: u16,
    height_n_tiles: u16,
    name: String,
}

impl MapLevelHeader {
    pub fn new(header_data: &[u8]) -> Self {
        assert_eq!(header_data.len(), 38);
        MapLevelHeader {
            // TODO can this be simplified by reading the entire struct based on its C types?
            offset_plane0: i32::from_le_bytes(header_data[0..4].try_into().unwrap()),
            offset_plane1: i32::from_le_bytes(header_data[4..8].try_into().unwrap()),
            offset_plane2: i32::from_le_bytes(header_data[8..12].try_into().unwrap()),
            length_plane0: u16::from_le_bytes(header_data[12..14].try_into().unwrap()),
            length_plane1: u16::from_le_bytes(header_data[14..16].try_into().unwrap()),
            length_plane2: u16::from_le_bytes(header_data[16..18].try_into().unwrap()),
            width_n_tiles: u16::from_le_bytes(header_data[18..20].try_into().unwrap()),
            height_n_tiles: u16::from_le_bytes(header_data[20..22].try_into().unwrap()),
            name: String::from_utf8(header_data[22..].to_owned())
                .expect("map level header name could not be parsed")
                .trim_end_matches(char::from(0))
                .to_owned(),
        }
    }
}

/// See: https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression
fn rlew_decompress(compressed_data: &[u8], magic_word: &[u8; 2]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut word_i = 0;
    let n_words_max = compressed_data.len() / 2;

    while word_i < n_words_max {
        let offset = word_i * 2;
        let word_bytes = &compressed_data[offset..(offset + 2)];
        if word_bytes == magic_word {
            if word_i + 1 == n_words_max {
                dbg!("malformed input?");
                break;
            }
            let count = u16::from_le_bytes(
                compressed_data[(offset + 2)..(offset + 4)]
                    .try_into()
                    .unwrap(),
            ) as usize;
            let value = &compressed_data[(offset + 4)..(offset + 6)];
            output.extend(vec![value; count].concat());
            word_i += 3;
        } else {
            output.extend_from_slice(&compressed_data[offset..(offset + 2)]);
            word_i += 1;
        }
    }
    // TODO: remove/revisit this ugly and inefficient hack for testing...
    output.into_iter().take(64 * 64 * 2).collect()
}

/// See: https://moddingwiki.shikadi.net/wiki/Carmack_compression
fn carmack_decompress(compressed_data: &[u8]) -> Vec<u8> {
    const NEAR_POINTER: u8 = 0xA7;
    const FAR_POINTER: u8 = 0xA8;
    let mut output = Vec::new();
    let mut word_i = 0;
    let mut n_shifts = 0;
    let mut offset = 0;

    while offset < compressed_data.len() - 2 {
        match &compressed_data[offset..(offset + 2)] {
            [0x00, NEAR_POINTER] | [0x00, FAR_POINTER] => {
                // ignore 0x00 and invert the following word
                output.push(compressed_data[offset + 2]);
                output.push(compressed_data[offset + 1]);
                n_shifts += 1;
            }
            [count, NEAR_POINTER] => {
                let distance = usize::from(compressed_data[offset + 2]);
                let segment_start = output.len() - distance * 2;
                let segment_end = segment_start + usize::from(*count) * 2;
                let segment_to_repeat = output[segment_start..segment_end].to_vec();
                output.extend_from_slice(&segment_to_repeat);
                n_shifts += 1;
            }
            [count, FAR_POINTER] => {
                let distance = u16::from_le_bytes(
                    compressed_data[(offset + 2)..(offset + 4)]
                        .try_into()
                        .unwrap(),
                );
                let segment_start = (usize::from(distance) + 1) * 2;
                let segment_end = segment_start + usize::from(*count) * 2;
                let segment_to_repeat = output[segment_start..segment_end].to_vec();
                output.extend_from_slice(&segment_to_repeat);
                word_i += 1;
            }
            word_bytes => {
                output.extend_from_slice(word_bytes);
            }
        }

        word_i += 1;
        offset = word_i * 2 + n_shifts;
    }

    if offset < compressed_data.len() {
        let remainder = &compressed_data[offset..];
        output.extend_from_slice(remainder);
    }

    output
}

fn get_plane(data: &[u8], offset: i32, length: u16, magic_rlew_word: &[u8; 2]) -> [[u16; 64]; 64] {
    let plane_start = offset as usize;
    let plane_end = plane_start + length as usize;
    let decarmackized = carmack_decompress(&data[plane_start..plane_end]);
    let bytes = rlew_decompress(&decarmackized[4..], magic_rlew_word);
    let mut bytes = bytes.chunks_exact(2).map(|word| u16::from_le_bytes(word.try_into().unwrap()));
    let mut result = [[0; HEIGHT]; WIDTH];
    for y in 0..HEIGHT {
        for x in result.iter_mut().take(WIDTH) {
            x[y] = bytes.next().unwrap();
        }
    }
    result
}

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
    name: String,
}

impl Map {
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
                    return (x, y, direction)
                }
            }
        }
        panic!("Can't find the player in the map");
    }
}

fn parse_map_data<P: AsRef<Path>>(path: P, meta: MapHead) -> Vec<Map> {
    let raw_data = fs::read(path).expect("could not read GAMEMAPS file");
    let mut maps = Vec::new();

    for pointer in meta.pointers {
        let pointer = pointer as usize;
        let header = MapLevelHeader::new(&raw_data[pointer..(pointer + 38)]);

        if header.offset_plane0 == 0 {
            continue;
        }

        assert_eq!(64, header.width_n_tiles);
        assert_eq!(64, header.height_n_tiles);

        // plane2 is unused in wolf, skipping
        maps.push(Map {
            plane0: get_plane(
                &raw_data,
                header.offset_plane0,
                header.length_plane0,
                &meta.magic,
            ),
            plane1: get_plane(
                &raw_data,
                header.offset_plane1,
                header.length_plane1,
                &meta.magic,
            ),
            name: header.name,
        });
    }

    maps
}

// FIXME improve this loading interface
/// Made with MAPHEAD.WL1 and GAMEMAPS.WL1 in mind
pub fn load_maps<P: AsRef<Path>>(maphead: P, gamemaps: P, keep_n_first: Option<usize>) -> Vec<Map> {
    let metadata = parse_map_head(maphead, keep_n_first);
    parse_map_data(gamemaps, metadata)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_rlew_decompress() {
        // marcolugo@MARCO-LUGO bin % echo -n "\x00\x01\x03\x04\xFE\xFE\x05\x00\xA0\x0A" > test3.bin
        // marcolugo@MARCO-LUGO bin % node gamecomp.js -cmp-rlew-id < test3.bin > rlew_expanded.bin
        // marcolugo@MARCO-LUGO bin % xxd rlew_expanded.bin
        // 00000000: 0001 0304 a00a a00a a00a a00a a00a       ..............
        assert_eq!(
            rlew_decompress(
                &[0x00, 0x01, 0x03, 0x04, 0xFE, 0xFE, 0x05, 0x00, 0xA0, 0x0A],
                &[0xFE, 0xFE]
            ),
            &[0x00, 0x01, 0x03, 0x04, 0xA0, 0x0A, 0xA0, 0x0A, 0xA0, 0x0A, 0xA0, 0x0A, 0xA0, 0x0A]
        );
    }

    #[test]
    fn test_carmack_decompress_escaped() {
        // from the provided example: https://moddingwiki.shikadi.net/wiki/Carmack_compression
        // marcolugo@MARCO-LUGO bin % echo -n "\x00\xA7\x12\xEE\xFF\x00\xA8\x34\xCC\xDD" > test0.bin
        // marcolugo@MARCO-LUGO bin % node gamecomp.js -cmp-carmackize < test0.bin > decarmackized0.bin
        // marcolugo@MARCO-LUGO bin % xxd decarmackized0.bin
        // 00000000: 12a7 eeff 34a8 ccdd                      ....4...
        assert_eq!(
            carmack_decompress(&[0x00, 0xA7, 0x12, 0xEE, 0xFF, 0x00, 0xA8, 0x34, 0xCC, 0xDD]),
            &[0x12, 0xA7, 0xEE, 0xFF, 0x34, 0xA8, 0xCC, 0xDD]
        );
    }

    #[test]
    fn test_carmack_decompress_near_pointer() {
        // marcolugo@MARCO-LUGO bin % echo -n "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x04\xA7\x06\x00\x01" > test.bin
        // marcolugo@MARCO-LUGO bin % node gamecomp.js -cmp-carmackize < test.bin > decarmackized.bin
        // marcolugo@MARCO-LUGO bin % xxd decarmackized.bin
        // 00000000: 0001 0203 0405 0607 0809 0a0b 0001 0203  ................
        // 00000010: 0405 0607 0001                           ......
        assert_eq!(
            carmack_decompress(&[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x04, 0xA7,
                0x06, 0x00, 0x01
            ]),
            &[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x00, 0x01,
                0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn test_carmack_decompress() {
        // marcolugo@MARCO-LUGO bin % echo -n "\xEA\xEB\xEC\x00\xA7\x14\xDE\xFA\x00\xA8\x34\xCC\xDD\xAB\x01\x03\xA7\x07\x00\x02\xA8\x0A\x00\x02\x02\x03\xFF\x0A\x2A\x00\xA7\xFF" > test5.bin
        // marcolugo@MARCO-LUGO bin % node gamecomp.js -cmp-carmackize < test5.bin > decarmackized5.bin
        // marcolugo@MARCO-LUGO bin % xxd decarmackized5.bin
        // 00000000: eaeb ec00 a714 defa 34a8 ccdd ab01 eaeb  ........4.......
        // 00000010: ec00 a714 0002 a80a 0002 0203 ff0a 2a00  ..............*.
        // 00000020: a7ff                                     ..
        assert_eq!(
            carmack_decompress(&[
                0xEA, 0xEB, 0xEC, 0x00, 0xA7, 0x14, 0xDE, 0xFA, 0x00, 0xA8, 0x34, 0xCC, 0xDD, 0xAB,
                0x01, 0x03, 0xA7, 0x07, 0x00, 0x02, 0xA8, 0x0A, 0x00, 0x02, 0x02, 0x03, 0xFF, 0x0A,
                0x2A, 0x00, 0xA7, 0xFF
            ]),
            &[
                0xEA, 0xEB, 0xEC, 0x00, 0xA7, 0x14, 0xDE, 0xFA, 0x34, 0xA8, 0xCC, 0xDD, 0xAB, 0x01,
                0xEA, 0xEB, 0xEC, 0x00, 0xA7, 0x14, 0x00, 0x02, 0xA8, 0x0A, 0x00, 0x02, 0x02, 0x03,
                0xFF, 0x0A, 0x2A, 0x00, 0xA7, 0xFF
            ]
        );
    }

    // ignoring this one as it depends on the data files
    #[test]
    #[ignore]
    fn map_file_parsing() {
        let maps = load_maps("data/MAPHEAD.WL1", "data/GAMEMAPS.WL1", Some(2));
        assert_eq!("Wolf1 Map1", maps[0].name);
        assert_eq!("Wolf1 Map2", maps[1].name);
    }

    #[test]
    #[ignore]
    fn dump_map0_plane0_printout() {
        let maps = load_maps("data/MAPHEAD.WL1", "data/GAMEMAPS.WL1", Some(1));
        let mut file = fs::File::create("test_map0.txt").unwrap();
        write!(file, "{}", maps[0]).unwrap();
    }
}
