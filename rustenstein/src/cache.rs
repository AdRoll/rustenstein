use std::fs;

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

const DATADIR: &str = "shareware";

pub struct Cache {
    pics: Vec<Picture>,
    textures: Vec<Vec<u8>>,
    sprites: Vec<(CompShape,Vec<u8>)>,
    sounds: Vec<Vec<u8>>,
}

impl Cache {
    pub fn new(pics: Vec<Picture>, textures: Vec<Vec<u8>>, sprites: Vec<(CompShape,Vec<u8>)>, sounds: Vec<Vec<u8>>) -> Cache {
        Cache {
            pics: pics,
            textures: textures,
            sprites: sprites,
            sounds: sounds,
        }
    }

    pub fn get_pic(&self, index: usize) -> &Picture {
        &self.pics[index - 3]
    }

    pub fn get_texture(&self, index: usize) -> &Vec<u8> {
        &self.textures[index]
    }

    pub fn get_sprite(&self, index: usize) -> &(CompShape,Vec<u8>) {
        &self.sprites[index]
    }

    pub fn get_sound(&self, index: usize) -> &Vec<u8> {
        &self.sounds[index]
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

pub fn startup() -> Cache {
    setup_graphics()
}

fn setup_graphics() -> Cache {
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
            data: data,
        })
    }

    let vswap_file = fs::read(DATADIR.to_owned() + "/VSWAP.WL1")
        .expect("Something went wrong reading the file");

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
    let mut sprites: Vec<(CompShape,Vec<u8>)> = Vec::new();
    let mut sounds: Vec<Vec<u8>> = Vec::new();

    for i in 0..chunks_in_file - 1 { // last value fails as it seems length is wrong
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
            if value.len() > 0 {
                sprites.push((CompShape {
                        left_pix: u16::from_le_bytes([value[0], value[1]]),
                        right_pix: u16::from_le_bytes([value[2], value[3]]),
                        dataofs: value[4..132]
                                    .chunks_exact(2)
                                    .into_iter()
                                    .map(|a| u16::from_le_bytes([a[0], a[1]]))
                                    .collect()
                    },
                    value.to_vec()));
            }
        } else {
            sounds.push(value.to_vec());
        }
    }

    Cache::new(pics, textures, sprites, sounds)
}

fn huff_expand(huff: &Vec<(u16, u16)>, source: &[u8], length: usize) -> Vec<u8> {
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
            mask = mask << 1;
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

fn load_graphic(
    source: &[u8],
    headers: &Vec<u32>,
    huff: &Vec<(u16, u16)>,
    chunk: usize,
) -> Vec<u8> {
    let pos = headers[chunk] as usize;
    let end = headers[chunk + 1] as usize;

    let length = u32::from_le_bytes(source[pos..pos + 4].try_into().unwrap());
    huff_expand(&huff, &source[pos + 4..end], length.try_into().unwrap())
}
