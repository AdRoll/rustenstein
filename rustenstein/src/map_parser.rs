use std::fs;
use std::path::PathBuf;

// see some map plans here: https://wolfenstein.fandom.com/wiki/Wolfenstein_3D
// some map format info: https://moddingwiki.shikadi.net/wiki/GameMaps_Format
// on the RLEW compression algorithm: https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression

// first goal is to get the job done, we'll care about efficiency later and even then,
// the parsing is so small that it should not really matter for our purposes anyways

#[derive(Debug)]
struct MapHead {
    magic: u16,
    pointers: Vec<i32>,
    title: Vec<u8>, // so far empty in map files we've seen, no idea what to do with this for now... could probably be a string
}

fn parse_map_head(path: PathBuf) -> MapHead {
    let raw_data = fs::read(path).expect("could not read MAPHEAD file");

    MapHead {
        magic: u16::from_le_bytes(raw_data[0..2].try_into().unwrap()),
        pointers: raw_data[2..(4 * 100)]
            .chunks_exact(4)
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
fn rlew_decompress(compressed_data: &[u8]) -> Vec<u8> {
    // RLEW signature for our WL1 file is 0xABCD in little-endian
    let mut output = Vec::new();
    let mut word_i = 0;
    let n_words = compressed_data.len() / 2;

    while word_i < n_words {
        let offset = word_i * 2;
        if compressed_data[offset..(offset + 2)] == [0xCD, 0xAB] {
            let count = u16::from_le_bytes(
                compressed_data[(offset + 2)..(offset + 4)]
                    .try_into()
                    .unwrap(),
            ) as usize;
            output.extend(vec![&compressed_data[(offset + 2)..(offset + 4)]; count].concat());
            word_i += 3;
        } else {
            output.extend_from_slice(&compressed_data[offset..(offset + 2)]);
            word_i += 1;
        }
    }

    output
}

fn get_plane(data: &[u8], offset: i32, length: u16) -> Option<Vec<u8>> {
    if offset > 0 {
        let plane_start = offset as usize;
        let plane_end = plane_start + length as usize;
        Some(rlew_decompress(&data[plane_start..plane_end]))
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Map {
    plane0: Option<Vec<u8>>,
    plane1: Option<Vec<u8>>,
    plane2: Option<Vec<u8>>,
    width_n_tiles: u16,
    height_n_tiles: u16,
    name: String,
}

fn parse_map_data(path: PathBuf, metadata: MapHead) -> Vec<Map> {
    let raw_data = fs::read(path).expect("could not read GAMEMAPS file");
    let mut maps = Vec::new();

    for pointer in metadata.pointers {
        let pointer = pointer as usize;
        let header = MapLevelHeader::new(&raw_data[pointer..(pointer + 38)]);
        // dbg!(&header);
        maps.push(Map {
            plane0: get_plane(&raw_data, header.offset_plane0, header.length_plane0),
            plane1: get_plane(&raw_data, header.offset_plane1, header.length_plane1),
            plane2: get_plane(&raw_data, header.offset_plane2, header.length_plane2),
            width_n_tiles: header.width_n_tiles,
            height_n_tiles: header.height_n_tiles,
            name: header.name,
        });
    }

    maps
}

/// Made with MAPHEAD.WL1 and GAMEMAPS.WL1 in mind
pub fn load_maps(maphead: PathBuf, gamemaps: PathBuf) -> Vec<Map> {
    let metadata = parse_map_head(maphead);
    parse_map_data(gamemaps, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_map0_plane0() {
        let maps = load_maps(
            "shareware/MAPHEAD.WL1".into(),
            "shareware/GAMEMAPS.WL1".into(),
        );
        fs::write("test_plane0.bin", maps[0].plane0.as_ref().unwrap()).unwrap();
    }
}
