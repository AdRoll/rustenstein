use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::map;
use crate::map::Map;
use std::fs;
use std::path::Path;

pub const H_BJPIC: usize = 3;
pub const H_CASTLEPIC: usize = 4;
pub const H_KEYBOARDPIC: usize = 5;
pub const H_JOYPIC: usize = 6;
pub const H_HEALPIC: usize = 7;
pub const H_TREASUREPIC: usize = 8;
pub const H_GUNPIC: usize = 9;
pub const H_KEYPIC: usize = 10;
pub const H_BLAZEPIC: usize = 11;
pub const H_WEAPON1234PIC: usize = 12;
pub const H_WOLFLOGOPIC: usize = 13;
pub const H_VISAPIC: usize = 14;
pub const H_MCPIC: usize = 15;
pub const H_IDLOGOPIC: usize = 16;
pub const H_TOPWINDOWPIC: usize = 17;
pub const H_LEFTWINDOWPIC: usize = 18;
pub const H_RIGHTWINDOWPIC: usize = 19;
pub const H_BOTTOMINFOPIC: usize = 20;
pub const H_SPEARADPIC: usize = 21;
pub const C_OPTIONSPIC: usize = 22;
pub const C_CURSOR1PIC: usize = 23;
pub const C_CURSOR2PIC: usize = 24;
pub const C_NOTSELECTEDPIC: usize = 25;
pub const C_SELECTEDPIC: usize = 26;
pub const C_FXTITLEPIC: usize = 27;
pub const C_DIGITITLEPIC: usize = 28;
pub const C_MUSICTITLEPIC: usize = 29;
pub const C_MOUSELBACKPIC: usize = 30;
pub const C_BABYMODEPIC: usize = 31;
pub const C_EASYPIC: usize = 32;
pub const C_NORMALPIC: usize = 33;
pub const C_HARDPIC: usize = 34;
pub const C_LOADSAVEDISKPIC: usize = 35;
pub const C_DISKLOADING1PIC: usize = 36;
pub const C_DISKLOADING2PIC: usize = 37;
pub const C_CONTROLPIC: usize = 38;
pub const C_CUSTOMIZEPIC: usize = 39;
pub const C_LOADGAMEPIC: usize = 40;
pub const C_SAVEGAMEPIC: usize = 41;
pub const C_EPISODE1PIC: usize = 42;
pub const C_EPISODE2PIC: usize = 43;
pub const C_EPISODE3PIC: usize = 44;
pub const C_EPISODE4PIC: usize = 45;
pub const C_EPISODE5PIC: usize = 46;
pub const C_EPISODE6PIC: usize = 47;
pub const C_CODEPIC: usize = 48;
pub const C_TIMECODEPIC: usize = 49;
pub const C_LEVELPIC: usize = 50;
pub const C_NAMEPIC: usize = 51;
pub const C_SCOREPIC: usize = 52;
pub const C_JOY1PIC: usize = 53;
pub const C_JOY2PIC: usize = 54;
pub const L_GUYPIC: usize = 55;
pub const L_COLONPIC: usize = 56;
pub const L_NUM0PIC: usize = 57;
pub const L_NUM1PIC: usize = 58;
pub const L_NUM2PIC: usize = 59;
pub const L_NUM3PIC: usize = 60;
pub const L_NUM4PIC: usize = 61;
pub const L_NUM5PIC: usize = 62;
pub const L_NUM6PIC: usize = 63;
pub const L_NUM7PIC: usize = 64;
pub const L_NUM8PIC: usize = 65;
pub const L_NUM9PIC: usize = 66;
pub const L_PERCENTPIC: usize = 67;
pub const L_APIC: usize = 68;
pub const L_BPIC: usize = 69;
pub const L_CPIC: usize = 70;
pub const L_DPIC: usize = 71;
pub const L_EPIC: usize = 72;
pub const L_FPIC: usize = 73;
pub const L_GPIC: usize = 74;
pub const L_HPIC: usize = 75;
pub const L_IPIC: usize = 76;
pub const L_JPIC: usize = 77;
pub const L_KPIC: usize = 78;
pub const L_LPIC: usize = 79;
pub const L_MPIC: usize = 80;
pub const L_NPIC: usize = 81;
pub const L_OPIC: usize = 82;
pub const L_PPIC: usize = 83;
pub const L_QPIC: usize = 84;
pub const L_RPIC: usize = 85;
pub const L_SPIC: usize = 86;
pub const L_TPIC: usize = 87;
pub const L_UPIC: usize = 88;
pub const L_VPIC: usize = 89;
pub const L_WPIC: usize = 90;
pub const L_XPIC: usize = 91;
pub const L_YPIC: usize = 92;
pub const L_ZPIC: usize = 93;
pub const L_EXPOINTPIC: usize = 94;
pub const L_APOSTROPHEPIC: usize = 95;
pub const L_GUY2PIC: usize = 96;
pub const L_BJWINSPIC: usize = 97;
pub const STATUSBARPIC: usize = 98;
pub const TITLEPIC: usize = 99;
pub const PG13PIC: usize = 100;
pub const CREDITSPIC: usize = 101;
pub const HIGHSCORESPIC: usize = 102;
pub const KNIFEPIC: usize = 103;
pub const GUNPIC: usize = 104;
pub const MACHINEGUNPIC: usize = 105;
pub const GATLINGGUNPIC: usize = 106;
pub const NOKEYPIC: usize = 107;
pub const GOLDKEYPIC: usize = 108;
pub const SILVERKEYPIC: usize = 109;
pub const N_BLANKPIC: usize = 110;
pub const N_0PIC: usize = 111;
pub const N_1PIC: usize = 112;
pub const N_2PIC: usize = 113;
pub const N_3PIC: usize = 114;
pub const N_4PIC: usize = 115;
pub const N_5PIC: usize = 116;
pub const N_6PIC: usize = 117;
pub const N_7PIC: usize = 118;
pub const N_8PIC: usize = 119;
pub const N_9PIC: usize = 120;
pub const FACE1APIC: usize = 121;
pub const FACE1BPIC: usize = 122;
pub const FACE1CPIC: usize = 123;
pub const FACE2APIC: usize = 124;
pub const FACE2BPIC: usize = 125;
pub const FACE2CPIC: usize = 126;
pub const FACE3APIC: usize = 127;
pub const FACE3BPIC: usize = 128;
pub const FACE3CPIC: usize = 129;
pub const FACE4APIC: usize = 130;
pub const FACE4BPIC: usize = 131;
pub const FACE4CPIC: usize = 132;
pub const FACE5APIC: usize = 133;
pub const FACE5BPIC: usize = 134;
pub const FACE5CPIC: usize = 135;
pub const FACE6APIC: usize = 136;
pub const FACE6BPIC: usize = 137;
pub const FACE6CPIC: usize = 138;
pub const FACE7APIC: usize = 139;
pub const FACE7BPIC: usize = 140;
pub const FACE7CPIC: usize = 141;
pub const FACE8APIC: usize = 142;
pub const GOTGATLINGPIC: usize = 143;
pub const MUTANTBJPIC: usize = 144;
pub const PAUSEDPIC: usize = 145;
pub const GETPSYCHEDPIC: usize = 146;

pub const NUMCHUNKS: u32 = 161;
pub const NUMPICS: usize = 144;
pub const STARTPICS: usize = 3;

const NUM_MAPS: usize = 60;

const DATADIR: &str = "data";

pub struct Cache {
    pics: Vec<Picture>,
    textures: Vec<Vec<u8>>,
    sprites: Vec<(CompShape, Vec<u8>)>,
    sounds: Vec<Vec<u8>>,
    maps: Vec<Map>,
}

impl Cache {
    pub fn new(
        pics: Vec<Picture>,
        textures: Vec<Vec<u8>>,
        sprites: Vec<(CompShape, Vec<u8>)>,
        sounds: Vec<Vec<u8>>,
        maps: Vec<Map>,
    ) -> Cache {
        Cache {
            pics,
            textures,
            sprites,
            sounds,
            maps,
        }
    }

    pub fn get_pic(&self, index: usize) -> &Picture {
        &self.pics[index - 3]
    }

    pub fn get_texture(&self, index: usize) -> &Vec<u8> {
        &self.textures[index]
    }

    pub fn get_sprite(&self, index: usize) -> &(CompShape, Vec<u8>) {
        &self.sprites[index]
    }

    pub fn get_sound(&self, index: usize) -> &Vec<u8> {
        &self.sounds[index]
    }

    pub fn get_map(&self, episode: usize, level: usize) -> &Map {
        &self.maps[level + 10 * episode]
    }
}

pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct CompShape {
    pub left_pix: u16,
    pub right_pix: u16,
    pub dataofs: Vec<u16>,
}

pub fn init() -> Cache {
    let huff_file = fs::read(DATADIR.to_owned() + "/VGADICT.WL1")
        .expect("Something went wrong reading the file");

    let mut huff: Vec<(u16, u16)> = Vec::new();

    for i in huff_file.chunks_exact(4) {
        let bit0 = u16::from_le_bytes([i[0], i[1]]);
        let bit1 = u16::from_le_bytes([i[2], i[3]]);
        huff.push((bit0, bit1));
    }

    let headers_file = fs::read(DATADIR.to_owned() + "/VGAHEAD.WL1")
        .expect("Something went wrong reading the file");

    let mut buffer = [0u8; 4];
    let mut headers: Vec<u32> = Vec::new();

    for i in headers_file.chunks_exact(3) {
        buffer[..3].copy_from_slice(i);
        headers.push(u32::from_le_bytes(buffer));
    }

    let graph_file = fs::read(DATADIR.to_owned() + "/VGAGRAPH.WL1")
        .expect("Something went wrong reading the file");

    let a = headers[1] as usize;
    let pictable_bytes = huff_expand(&huff, &graph_file[4..a], (NUMPICS + 3) * 4);

    let mut pics: Vec<Picture> = Vec::new();

    for chunk in STARTPICS..GETPSYCHEDPIC + 1 {
        let i = (chunk - STARTPICS) * 4;
        let width = u16::from_le_bytes([pictable_bytes[i], pictable_bytes[i + 1]]);
        let height = u16::from_le_bytes([pictable_bytes[i + 2], pictable_bytes[i + 3]]);
        let data = load_graphic(&graph_file, &headers, &huff, chunk);

        pics.push(Picture {
            width: width as u32,
            height: height as u32,
            data,
        })
    }

    let vswap_file =
        fs::read(DATADIR.to_owned() + "/VSWAP.WL1").expect("Something went wrong reading the file");

    let chunks_in_file = u16::from_le_bytes([vswap_file[0], vswap_file[1]]) as usize;
    let pm_sprite_start = u16::from_le_bytes([vswap_file[2], vswap_file[3]]) as usize;
    let pm_sound_start = u16::from_le_bytes([vswap_file[4], vswap_file[5]]) as usize;
    let mut page_offsets: Vec<u32> = Vec::new();
    let mut page_lengths: Vec<u16> = Vec::new();
    let offsets_start = 6;
    let offsets_end = offsets_start + 4 * (chunks_in_file + 1);
    let lengths_start = offsets_end;
    let lengths_end = lengths_start + 2 * chunks_in_file;

    for i in vswap_file[offsets_start..offsets_end].chunks_exact(4) {
        let offset = u32::from_le_bytes(i.try_into().unwrap());
        page_offsets.push(offset);
    }

    for i in vswap_file[lengths_start..lengths_end].chunks_exact(2) {
        let length = u16::from_le_bytes(i.try_into().unwrap());
        page_lengths.push(length);
    }

    let mut textures: Vec<Vec<u8>> = Vec::new();
    let mut sprites: Vec<(CompShape, Vec<u8>)> = Vec::new();
    let mut sounds: Vec<Vec<u8>> = Vec::new();

    for i in 0..chunks_in_file - 1 {
        // last value fails as it seems length is wrong
        if page_offsets[i] == 0 {
            // sparse page
            continue;
        }

        let mut value_end = page_offsets[i + 1] as usize;
        if page_offsets[i + 1] == 0 {
            value_end = page_offsets[i] as usize + page_lengths[i] as usize;
        }

        let value = &vswap_file[page_offsets[i] as usize..value_end];
        if i < pm_sprite_start {
            textures.push(value.to_vec());
        } else if i < pm_sound_start {
            // for sprites we parse the CompShape struct as well
            if !value.is_empty() {
                sprites.push((
                    CompShape {
                        left_pix: u16::from_le_bytes([value[0], value[1]]),
                        right_pix: u16::from_le_bytes([value[2], value[3]]),
                        dataofs: value[4..132]
                            .chunks_exact(2)
                            .into_iter()
                            .map(|a| u16::from_le_bytes([a[0], a[1]]))
                            .collect(),
                    },
                    value.to_vec(),
                ));
            }
        } else {
            sounds.push(value.to_vec());
        }
    }

    let maps = load_maps();

    Cache::new(pics, textures, sprites, sounds, maps)
}

fn huff_expand(huff: &[(u16, u16)], source: &[u8], length: usize) -> Vec<u8> {
    let mut dest: Vec<u8> = Vec::new();
    let headptr = 254; // head node is always node 254
    let mut huffptr = headptr;
    let mut i = 0;

    let mut mask = 1;
    let mut nodeval;

    loop {
        if (source[i] & mask) == 0 {
            nodeval = huff[huffptr].0;
        } else {
            nodeval = huff[huffptr].1;
        }
        if mask == 0x80 {
            i += 1;
            mask = 1;
        } else {
            mask <<= 1;
        }
        if nodeval < 256 {
            dest.push(nodeval.try_into().unwrap());
            huffptr = headptr;
            if dest.len() >= length {
                break;
            }
        } else {
            huffptr = nodeval as usize - 256;
        }
    }
    dest
}

fn load_graphic(source: &[u8], headers: &[u32], huff: &[(u16, u16)], chunk: usize) -> Vec<u8> {
    let pos = headers[chunk] as usize;
    let end = headers[chunk + 1] as usize;

    let length = u32::from_le_bytes(source[pos..pos + 4].try_into().unwrap());
    huff_expand(huff, &source[pos + 4..end], length.try_into().unwrap())
}

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

fn load_maps() -> Vec<Map> {
    let map_metadata = parse_map_head(DATADIR.to_owned() + "/MAPHEAD.WL1");
    parse_map_data(DATADIR.to_owned() + "/GAMEMAPS.WL1", map_metadata)
}

fn parse_map_head<P: AsRef<Path>>(path: P) -> MapHead {
    let raw_data = fs::read(path).expect("could not read MAPHEAD file");

    MapHead {
        magic: raw_data[0..2].try_into().unwrap(),
        pointers: raw_data[2..=(4 * 100)]
            .chunks_exact(4)
            .take(NUM_MAPS)
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
    let mut bytes = bytes
        .chunks_exact(2)
        .map(|word| u16::from_le_bytes(word.try_into().unwrap()));
    let mut result = [[0; MAP_HEIGHT]; MAP_WIDTH];
    for y in 0..MAP_HEIGHT {
        for x in result.iter_mut().take(MAP_WIDTH) {
            x[y] = bytes.next().unwrap();
        }
    }
    result
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
        maps.push(Map::new(
            get_plane(
                &raw_data,
                header.offset_plane0,
                header.length_plane0,
                &meta.magic,
            ),
            get_plane(
                &raw_data,
                header.offset_plane1,
                header.length_plane1,
                &meta.magic,
            ),
            header.name,
        ));
    }
    maps
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
        let maps = load_maps();
        assert_eq!("Wolf1 Map1", maps[0].name);
        assert_eq!("Wolf1 Map2", maps[1].name);
    }

    #[test]
    #[ignore]
    fn dump_map0_plane0_printout() {
        let maps = load_maps();
        let mut file = fs::File::create("test_map0.txt").unwrap();
        write!(file, "{}", maps[0]).unwrap();
    }
}
