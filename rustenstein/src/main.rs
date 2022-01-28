extern crate sdl2;

use cache::Picture;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, RenderTarget};
use sdl2::video::WindowContext;

mod cache;
mod map_data;
type ColorMap = [(u8, u8, u8); 256];
mod input_manager;
mod map_parser;
mod ray_caster;

use crate::ray_caster::RayHit;

const VGA_FLOOR_COLOR: usize = 0x19;
const VGA_CEILING_COLORS: [usize; 60] = [
    0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0xbf, 0x4e, 0x4e, 0x4e, 0x1d, 0x8d, 0x4e,
    0x1d, 0x2d, 0x1d, 0x8d, 0x1d, 0x1d, 0x1d, 0x1d, 0x1d, 0x2d, 0xdd, 0x1d, 0x1d, 0x98, 0x1d, 0x9d,
    0x2d, 0xdd, 0xdd, 0x9d, 0x2d, 0x4d, 0x1d, 0xdd, 0x7d, 0x1d, 0x2d, 0x2d, 0xdd, 0xd7, 0x1d, 0x1d,
    0x1d, 0x2d, 0x1d, 0x1d, 0x1d, 0x1d, 0xdd, 0xdd, 0x7d, 0xdd, 0xdd, 0xdd,
];

const STATUS_LINES: u32 = 40;

pub fn main() {
    let pics_cache = init();
    let (width, height, pix_width) = (960, 600, 320);
    let scale_factor = width / pix_width;
    let view_height = height - STATUS_LINES * scale_factor;
    let view_center = view_height / 2;
    let pix_height = view_height / scale_factor;
    let pix_center = view_height / scale_factor / 2;

    let level = 0;
    let sdl_context = sdl2::init().unwrap();
    //let mut input_manager = input_manager::InputManager::startup(&sdl_context);
    let mut ray_caster = ray_caster::RayCaster::init(&sdl_context, 470.0, 920.0, 1.54, pix_width, pix_height);
    let video_subsystem = sdl_context.video().unwrap();
    // let mut event_pump = sdl_context.event_pump().unwrap();

    let color_map = build_color_map();
    let titlepic = pics_cache.get_pic(cache::TITLEPIC);
    let statuspic = pics_cache.get_pic(cache::STATUSBARPIC);

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
    draw_to_texture(&mut texture, &titlepic, color_map);

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    //input_manager.wait_for_key();
    //let mut control_info = input_manager::ControlInfo::default();

    'main_loop: loop {
        //input_manager.read_control(&mut control_info);

        //if input_manager.should_exit() {
        //    break 'main_loop;
        //}
        let ray_hits = match ray_caster.tick() {
            Ok(hits) => { hits },
            Err(message) => {
                println!("{}",message);
                break 'main_loop;
            }
        };

        // fake walls
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, pix_width, pix_height)
            .unwrap();

        // TODO reduce duplication
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                // draw floor and ceiling colors
                let floor = color_map[VGA_FLOOR_COLOR];
                let ceiling = color_map[VGA_CEILING_COLORS[level]];

                for x in 0..pix_width {
                    for y in 0..pix_height / 2 {
                        put_pixel(buffer, pitch, x, y, ceiling);
                    }
                    for y in pix_height / 2..pix_height {
                        put_pixel(buffer, pitch, x, y, floor);
                    }
                }

                for x in 0..pix_width {
                    let color = if ray_hits[x as usize].horizontal {
                        color_map[150]
                    } else {
                        color_map[155]
                    };
                    let current = match ray_hits[x as usize].height {
                        rh if rh > pix_center => { pix_center },
                        rh => { rh }
                    };
                    for y in pix_center - current..pix_center + current {
                        put_pixel(buffer, pitch, x, y, color);
                    }
                }
            })
            .unwrap();

        canvas
            .copy(&texture, None, Rect::new(0, 0, width, view_height))
            .unwrap();

        // show status picture
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 320, 200)
            .unwrap();
        draw_to_texture(&mut texture, &statuspic, color_map);
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
    }
}

fn draw_to_texture(texture: &mut Texture, pic: &Picture, color_map: ColorMap) {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        // different from the window size
        for y in 0..pic.height {
            for x in 0..pic.width {
                let source_index =
                    (y * (pic.width >> 2) + (x >> 2)) + (x & 3) * (pic.width >> 2) * pic.height;
                let color = pic.data[source_index as usize];
                put_pixel(buffer, pitch, x, y, color_map[color as usize]);
            }
        }
    });
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

fn init() -> cache::Cache {
    cache::startup()
}
