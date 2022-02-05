#![allow(dead_code)]
#![allow(unused_imports)]
extern crate sdl2;

use cache::Picture;
use core::slice::Iter;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{RenderTarget, Texture};
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::time::Instant;

mod cache;
type ColorMap = [(u8, u8, u8); 256];
mod input_manager;
mod map;
mod ray_caster;

use crate::ray_caster::RayHit;

const VGA_FLOOR_COLOR: usize = 0x19;
const VGA_CEILING_COLORS: [usize; 60] = [
    0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0xbf, 0x4e, 0x4e, 0x4e, 0x1d, 0x8d, 0x4e,
    0x1d, 0x2d, 0x1d, 0x8d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x2d, 0xdd, 0x1d, 0x1d, 0x98, 0x1d, 0x9d,
    0x2d, 0xdd, 0xdd, 0x9d, 0x2d, 0x4d, 0x1d, 0xdd, 0x7d, 0x1d, 0x2d, 0x2d, 0xdd, 0xd7, 0x1d, 0x1d,
    0x1d, 0x2d, 0x1d, 0x1d, 0x1d, 0x1d, 0xdd, 0xdd, 0x7d, 0xdd, 0xdd, 0xdd,
];

const BASE_WIDTH: u32 = 320;
const BASE_HEIGHT: u32 = 200;
const STATUS_LINES: u32 = 40;
const PIX_WIDTH: u32 = BASE_WIDTH;
const PIX_HEIGHT: u32 = BASE_HEIGHT - STATUS_LINES;
const PIX_CENTER: u32 = PIX_HEIGHT / 2;
const DARKNESS: f64 = 0.75;

pub fn main() {
    let start_time = Instant::now();
    let cache = cache::init();
    let (width, height) = (960, 600);
    let scale_factor = width / PIX_WIDTH;
    let view_height = height - STATUS_LINES * scale_factor;
    let pix_center = view_height / scale_factor / 2;
    let sdl_context = sdl2::init().unwrap();
    //let mut input_manager = input_manager::InputManager::startup(&sdl_context);
    let video_subsystem = sdl_context.video().unwrap();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    let color_map = build_color_map();
    let titlepic = cache.get_pic(cache::TITLEPIC);
    let statuspic = cache.get_pic(cache::STATUSBARPIC);
    let default_facepic = cache.get_pic(cache::FACE1APIC);
    let lefteye_facepic = cache.get_pic(cache::FACE1BPIC);
    let righteye_facepic = cache.get_pic(cache::FACE1CPIC);
    let (weapon_shape, weapon_data) = cache.get_sprite(209);

    // we only support episode 0 for now -- the shareware one
    let episode = 0;
    let level = 0;
    let map = cache.get_map(episode, level);
    let mut ray_caster = ray_caster::RayCaster::init(&sdl_context, map, PIX_WIDTH, PIX_HEIGHT);

    let window = video_subsystem
        .window("rustenstein 3D", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 320, 200)
        .unwrap();
    draw_to_texture(&mut texture, titlepic, color_map);

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    ray_caster.tick(map).unwrap_or_default(); // TODO: can we ignore any error or do we need to handle it?
    ray_caster.wait_for_key(map);

    //input_manager.wait_for_key();
    //let mut control_info = input_manager::ControlInfo::default();

    'main_loop: loop {
        //input_manager.read_control(&mut control_info);

        //if input_manager.should_exit() {
        //    break 'main_loop;
        //}
        let ray_hits = match ray_caster.tick(map) {
            Ok(hits) => hits,
            Err(_) => {
                break 'main_loop;
            }
        };

        // fake walls
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, PIX_WIDTH, PIX_HEIGHT)
            .unwrap();

        // TODO reduce duplication
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                // draw floor and ceiling colors
                let floor = color_map[VGA_FLOOR_COLOR];
                let ceiling = color_map[VGA_CEILING_COLORS[level]];
                let vm = view_height / scale_factor / 2;

                for x in 0..PIX_WIDTH {
                    for y in 0..PIX_HEIGHT / 2 {
                        let ceilings = darken_color(ceiling, vm - y, pix_center);
                        put_pixel(buffer, pitch, x, y, ceilings);
                    }
                    for y in PIX_HEIGHT / 2..PIX_HEIGHT {
                        let floors = darken_color(floor, y - vm, pix_center);
                        put_pixel(buffer, pitch, x, y, floors);
                    }
                }

                for x in 0..PIX_WIDTH {
                    let mut color = if ray_hits[x as usize].horizontal {
                        color_map[150]
                    } else {
                        color_map[155]
                    };
                    let current = match ray_hits[x as usize].height {
                        rh if rh > pix_center => pix_center,
                        rh => rh,
                    };

                    // divide the color by a factor of the height to get a gradient shadow effect based on distance
                    color = darken_color(color, current, pix_center);

                    for y in pix_center - current..pix_center + current {
                        put_pixel(buffer, pitch, x, y, color);
                    }
                }

                simple_scale_shape(
                    PIX_WIDTH,
                    PIX_HEIGHT,
                    color_map,
                    buffer,
                    pitch,
                    weapon_shape.left_pix,
                    weapon_shape.right_pix,
                    &weapon_shape.dataofs,
                    weapon_data,
                );
            })
            .unwrap();

        canvas
            .copy(&texture, None, Rect::new(0, 0, width, view_height))
            .unwrap();

        // show status picture
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, BASE_WIDTH, BASE_HEIGHT)
            .unwrap();

        draw_to_texture(&mut texture, statuspic, color_map);

        let face_to_draw = match start_time.elapsed().as_secs() % 3 {
            0 => default_facepic,
            1 => lefteye_facepic,
            2 => righteye_facepic,
            _ => unreachable!(),
        };
        draw_face_to_texture(&mut texture, face_to_draw, color_map);

        // I don't know why I had to *5 for the height
        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    0,
                    view_height as i32,
                    width,
                    STATUS_LINES * scale_factor * 5,
                ),
            )
            .unwrap();

        canvas.present();
        // break 'main_loop;
    }
}

fn darken_color(color: (u8, u8, u8), lightness: u32, max: u32) -> (u8, u8, u8) {
    let (r, g, b) = color;
    let factor = lightness as f64 / max as f64 / DARKNESS;
    let rs = (r as f64 * factor) as u8;
    let gs = (g as f64 * factor) as u8;
    let bs = (b as f64 * factor) as u8;
    (rs, gs, bs)
}

// temporarily allowing too many arguments (default max is 7, we got 9)
// this function should probably be refactored
#[allow(clippy::too_many_arguments)]
fn simple_scale_shape(
    view_width: u32,
    view_height: u32,
    color_map: ColorMap,
    vbuf: &mut [u8],
    pitch: usize,
    left_pix: u16,
    right_pix: u16,
    dataofs: &[u16],
    shape_bytes: &[u8],
) {
    let sprite_scale_factor = 2;
    let xcenter = view_width / 2;
    let height = view_height + 1;

    let scale = height >> 1;
    let pixheight = scale * sprite_scale_factor;
    let actx = xcenter - scale;
    let upperedge = view_height / 2 - scale;
    // cmdptr=(word *) shape->dataofs;
    // cmdptr = iter(shape.dataofs)
    let mut cmdptr = dataofs.iter();

    let mut i = left_pix;
    let mut pixcnt = i as u32 * pixheight;
    let mut rpix = (pixcnt >> 6) + actx;

    while i <= right_pix {
        let mut lpix = rpix;
        if lpix >= view_width {
            break;
        }

        pixcnt += pixheight;
        rpix = (pixcnt >> 6) + actx;

        if lpix != rpix && rpix > 0 {
            if rpix > view_width {
                rpix = view_width;
                i = right_pix + 1;
            }
            let read_word = |line: &mut Iter<u8>| {
                u16::from_le_bytes([*line.next().unwrap_or(&0), *line.next().unwrap_or(&0)])
            };
            let read_word_signed = |line: &mut Iter<u8>| {
                i16::from_le_bytes([*line.next().unwrap_or(&0), *line.next().unwrap_or(&0)])
            };

            let cline = &shape_bytes[*cmdptr.next().unwrap() as usize..];
            while lpix < rpix {
                let mut line = cline.iter();
                let mut endy = read_word(&mut line);
                while endy > 0 {
                    endy >>= 1;
                    let newstart = read_word_signed(&mut line);
                    let starty = read_word(&mut line) >> 1;
                    let mut j = starty;
                    let mut ycnt = j as u32 * pixheight;
                    let mut screndy: i32 = (ycnt >> 6) as i32 + upperedge as i32;

                    let mut vmem_index: usize = if screndy < 0 {
                        lpix as usize * 3
                    } else {
                        screndy as usize * pitch + lpix as usize * 3
                    };

                    while j < endy {
                        let mut scrstarty = screndy;
                        ycnt += pixheight;
                        screndy = (ycnt >> 6) as i32 + upperedge as i32;
                        if scrstarty != screndy && screndy > 0 {
                            let index = newstart + j as i16;
                            let col = if index >= 0 {
                                shape_bytes[index as usize]
                            } else {
                                0
                            };
                            if scrstarty < 0 {
                                scrstarty = 0;
                            }
                            if screndy > view_height as i32 {
                                screndy = view_height as i32;
                                j = endy;
                            }

                            while scrstarty < screndy {
                                // FIXME can put pixel be used here instead?
                                let (r, g, b) = color_map[col as usize];
                                vbuf[vmem_index as usize] = r;
                                vbuf[vmem_index as usize + 1] = g;
                                vbuf[vmem_index as usize + 2] = b;
                                vmem_index += pitch;
                                scrstarty += 1;
                            }
                        }
                        j += 1;
                    }
                    endy = read_word(&mut line);
                }
                lpix += 1;
            }
        }
        i += 1;
    }
}

fn draw_to_texture(texture: &mut Texture, pic: &Picture, color_map: ColorMap) {
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // different from the window size
            for y in 0..pic.height {
                for x in 0..pic.width {
                    let source_index =
                        (y * (pic.width >> 2) + (x >> 2)) + (x & 3) * (pic.width >> 2) * pic.height;
                    let color = pic.data[source_index as usize];
                    put_pixel(buffer, pitch, x, y, color_map[color as usize]);
                }
            }
        })
        .unwrap_or_default(); // TODO: can we ignore any error or do we need to handle it?
}

fn draw_face_to_texture(texture: &mut Texture, pic: &Picture, color_map: ColorMap) {
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let shift_x = BASE_WIDTH / 2 - pic.width;
            let shift_y = pic.height / 8;
            // different from the window size
            for y in 0..pic.height {
                for x in 0..pic.width {
                    let source_index =
                        (y * (pic.width >> 2) + (x >> 2)) + (x & 3) * (pic.width >> 2) * pic.height;
                    let color = pic.data[source_index as usize];
                    put_pixel(
                        buffer,
                        pitch,
                        x + shift_x,
                        y + shift_y,
                        color_map[color as usize],
                    );
                }
            }
        })
        .unwrap_or_default(); // TODO: can we ignore any error or do we need to handle it?
}

fn put_pixel(buffer: &mut [u8], pitch: usize, x: u32, y: u32, color: (u8, u8, u8)) {
    let (r, g, b) = color;
    let offset = y as usize * pitch + x as usize * 3;
    buffer[offset] = r;
    buffer[offset + 1] = g;
    buffer[offset + 2] = b;
}

/// Returns an array of colors that maps indexes as used by wolf3d graphics
/// to r,g,b color tuples that can be used to write pixels into sdl surfaces/textures.
fn build_color_map() -> ColorMap {
    // [SDL_Color(r*255//63, g*255//63, b*255//63, 0) for r, g, b in COLORS]
    let palette = [
        (0, 0, 0),
        (0, 0, 42),
        (0, 42, 0),
        (0, 42, 42),
        (42, 0, 0),
        (42, 0, 42),
        (42, 21, 0),
        (42, 42, 42),
        (21, 21, 21),
        (21, 21, 63),
        (21, 63, 21),
        (21, 63, 63),
        (63, 21, 21),
        (63, 21, 63),
        (63, 63, 21),
        (63, 63, 63),
        (59, 59, 59),
        (55, 55, 55),
        (52, 52, 52),
        (48, 48, 48),
        (45, 45, 45),
        (42, 42, 42),
        (38, 38, 38),
        (35, 35, 35),
        (31, 31, 31),
        (28, 28, 28),
        (25, 25, 25),
        (21, 21, 21),
        (18, 18, 18),
        (14, 14, 14),
        (11, 11, 11),
        (8, 8, 8),
        (63, 0, 0),
        (59, 0, 0),
        (56, 0, 0),
        (53, 0, 0),
        (50, 0, 0),
        (47, 0, 0),
        (44, 0, 0),
        (41, 0, 0),
        (38, 0, 0),
        (34, 0, 0),
        (31, 0, 0),
        (28, 0, 0),
        (25, 0, 0),
        (22, 0, 0),
        (19, 0, 0),
        (16, 0, 0),
        (63, 54, 54),
        (63, 46, 46),
        (63, 39, 39),
        (63, 31, 31),
        (63, 23, 23),
        (63, 16, 16),
        (63, 8, 8),
        (63, 0, 0),
        (63, 42, 23),
        (63, 38, 16),
        (63, 34, 8),
        (63, 30, 0),
        (57, 27, 0),
        (51, 24, 0),
        (45, 21, 0),
        (39, 19, 0),
        (63, 63, 54),
        (63, 63, 46),
        (63, 63, 39),
        (63, 63, 31),
        (63, 62, 23),
        (63, 61, 16),
        (63, 61, 8),
        (63, 61, 0),
        (57, 54, 0),
        (51, 49, 0),
        (45, 43, 0),
        (39, 39, 0),
        (33, 33, 0),
        (28, 27, 0),
        (22, 21, 0),
        (16, 16, 0),
        (52, 63, 23),
        (49, 63, 16),
        (45, 63, 8),
        (40, 63, 0),
        (36, 57, 0),
        (32, 51, 0),
        (29, 45, 0),
        (24, 39, 0),
        (54, 63, 54),
        (47, 63, 46),
        (39, 63, 39),
        (32, 63, 31),
        (24, 63, 23),
        (16, 63, 16),
        (8, 63, 8),
        (0, 63, 0),
        (0, 63, 0),
        (0, 59, 0),
        (0, 56, 0),
        (0, 53, 0),
        (1, 50, 0),
        (1, 47, 0),
        (1, 44, 0),
        (1, 41, 0),
        (1, 38, 0),
        (1, 34, 0),
        (1, 31, 0),
        (1, 28, 0),
        (1, 25, 0),
        (1, 22, 0),
        (1, 19, 0),
        (1, 16, 0),
        (54, 63, 63),
        (46, 63, 63),
        (39, 63, 63),
        (31, 63, 62),
        (23, 63, 63),
        (16, 63, 63),
        (8, 63, 63),
        (0, 63, 63),
        (0, 57, 57),
        (0, 51, 51),
        (0, 45, 45),
        (0, 39, 39),
        (0, 33, 33),
        (0, 28, 28),
        (0, 22, 22),
        (0, 16, 16),
        (23, 47, 63),
        (16, 44, 63),
        (8, 42, 63),
        (0, 39, 63),
        (0, 35, 57),
        (0, 31, 51),
        (0, 27, 45),
        (0, 23, 39),
        (54, 54, 63),
        (46, 47, 63),
        (39, 39, 63),
        (31, 32, 63),
        (23, 24, 63),
        (16, 16, 63),
        (8, 9, 63),
        (0, 1, 63),
        (0, 0, 63),
        (0, 0, 59),
        (0, 0, 56),
        (0, 0, 53),
        (0, 0, 50),
        (0, 0, 47),
        (0, 0, 44),
        (0, 0, 41),
        (0, 0, 38),
        (0, 0, 34),
        (0, 0, 31),
        (0, 0, 28),
        (0, 0, 25),
        (0, 0, 22),
        (0, 0, 19),
        (0, 0, 16),
        (10, 10, 10),
        (63, 56, 13),
        (63, 53, 9),
        (63, 51, 6),
        (63, 48, 2),
        (63, 45, 0),
        (45, 8, 63),
        (42, 0, 63),
        (38, 0, 57),
        (32, 0, 51),
        (29, 0, 45),
        (24, 0, 39),
        (20, 0, 33),
        (17, 0, 28),
        (13, 0, 22),
        (10, 0, 16),
        (63, 54, 63),
        (63, 46, 63),
        (63, 39, 63),
        (63, 31, 63),
        (63, 23, 63),
        (63, 16, 63),
        (63, 8, 63),
        (63, 0, 63),
        (56, 0, 57),
        (50, 0, 51),
        (45, 0, 45),
        (39, 0, 39),
        (33, 0, 33),
        (27, 0, 28),
        (22, 0, 22),
        (16, 0, 16),
        (63, 58, 55),
        (63, 56, 52),
        (63, 54, 49),
        (63, 53, 47),
        (63, 51, 44),
        (63, 49, 41),
        (63, 47, 39),
        (63, 46, 36),
        (63, 44, 32),
        (63, 41, 28),
        (63, 39, 24),
        (60, 37, 23),
        (58, 35, 22),
        (55, 34, 21),
        (52, 32, 20),
        (50, 31, 19),
        (47, 30, 18),
        (45, 28, 17),
        (42, 26, 16),
        (40, 25, 15),
        (39, 24, 14),
        (36, 23, 13),
        (34, 22, 12),
        (32, 20, 11),
        (29, 19, 10),
        (27, 18, 9),
        (23, 16, 8),
        (21, 15, 7),
        (18, 14, 6),
        (16, 12, 6),
        (14, 11, 5),
        (10, 8, 3),
        (24, 0, 25),
        (0, 25, 25),
        (0, 24, 24),
        (0, 0, 7),
        (0, 0, 11),
        (12, 9, 4),
        (18, 0, 18),
        (20, 0, 20),
        (0, 0, 13),
        (7, 7, 7),
        (19, 19, 19),
        (23, 23, 23),
        (16, 16, 16),
        (12, 12, 12),
        (13, 13, 13),
        (54, 61, 61),
        (46, 58, 58),
        (39, 55, 55),
        (29, 50, 50),
        (18, 48, 48),
        (8, 45, 45),
        (8, 44, 44),
        (0, 41, 41),
        (0, 38, 38),
        (0, 35, 35),
        (0, 33, 33),
        (0, 31, 31),
        (0, 30, 30),
        (0, 29, 29),
        (0, 28, 28),
        (0, 27, 27),
        (38, 0, 34),
    ];
    palette.map(|(r, g, b)| {
        (
            (r * 255 / 63) as u8,
            (g * 255 / 63) as u8,
            (b * 255 / 63) as u8,
        )
    })
}
