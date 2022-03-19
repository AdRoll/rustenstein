extern crate sdl2;

use crate::constants::*;
use crate::map;
use crate::map::{Direction, Map, Tile};
use crate::player::Player;
use num::pow;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::RenderTarget;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::{EventPump, Sdl};
use std::cmp::min;
use std::f64::consts::PI;

const PLAYER_DIAM: i32 = 6;
const PLAYER_LEN: f64 = 40.0;
const FIELD_OF_VIEW: f64 = PI / 2.0;

const TILE_SIZE: f64 = 4.8;

// FIXME this is suspicious, probably use Option or Result?
struct Nothing;

pub struct RayCaster {
    canvas: WindowCanvas,
}

pub struct RayHit {
    pub height: u32,
    pub tile: u16,
    pub horizontal: bool,
    pub tex_x: usize,
}

impl RayCaster {
    pub fn init(sdl_context: &Sdl) -> RayCaster {
        let video_subsystem = sdl_context.video().unwrap();
        let window_2d = video_subsystem
            .window("", WIDTH_2D, HEIGHT_2D)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas_2d = window_2d.into_canvas().build().unwrap();
        canvas_2d.set_draw_color(Color::RGB(0, 255, 255));
        canvas_2d.clear();
        canvas_2d.present();

        RayCaster { canvas: canvas_2d }
    }

    pub fn tick(&mut self, n_rays: u32, height: u32, player: &Player, map: &Map) -> Vec<RayHit> {
        self.canvas.set_draw_color(Color::RGB(64, 64, 64));
        self.canvas.clear();
        draw_map(map, &mut self.canvas);
        let hits = draw_rays(n_rays, height, map, &mut self.canvas, player);
        draw_player(&mut self.canvas, player);
        self.canvas.present();
        hits
    }
}

fn draw_map<T: RenderTarget>(map: &Map, canvas: &mut Canvas<T>) {
    for y in 0..(MAP_HEIGHT) {
        for x in 0..(MAP_WIDTH) {
            //let i = (y * MAP_W + x) as usize;
            let color = match map.tile_at(x as u8, y as u8) {
                Tile::Wall(_) => Color::RGB(64, 64, 255),
                _ => Color::RGB(0, 0, 0),
            };
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(
                    (MAP_SCALE_W * x as u32 + 1).try_into().unwrap(),
                    (MAP_SCALE_H * y as u32 + 1).try_into().unwrap(),
                    MAP_SCALE_W - 1,
                    MAP_SCALE_H - 1,
                ))
                .unwrap();
        }
    }
}

fn draw_player<T: RenderTarget>(canvas: &mut Canvas<T>, player: &Player) {
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    let x = player.x.round() as i32;
    let y = player.y.round() as i32;
    let nx = (player.x + player.view_angle.sin() * PLAYER_LEN).round() as i32;
    let ny = (player.y + player.view_angle.cos() * PLAYER_LEN).round() as i32;
    canvas
        .fill_rect(Rect::new(
            x - PLAYER_DIAM,
            y - PLAYER_DIAM,
            PLAYER_DIAM as u32 * 2,
            PLAYER_DIAM as u32 * 2,
        ))
        .unwrap();
    canvas
        .draw_line(Point::new(x, y), Point::new(nx, ny))
        .unwrap();
}

fn draw_rays<T: RenderTarget>(
    n_rays: u32,
    height: u32,
    map: &Map,
    canvas: &mut Canvas<T>,
    player: &Player,
) -> Vec<RayHit> {
    let fov_delta = FIELD_OF_VIEW / (n_rays as f64);
    let mut hits: Vec<RayHit> = Vec::new();
    for i in 0..n_rays {
        let fov_angle = fov_delta * (i as f64);
        // transformation from cylindrical screen to flat screen (prevents fisheye effect)
        let offset = (FIELD_OF_VIEW / 2.0 - fov_angle).atan();

        let ray_h = cast_ray_h(map, canvas, player, offset);
        let ray_v = cast_ray_v(map, canvas, player, offset);
        let (hit, horiz) = match (ray_h, ray_v) {
            ((_, _, d1, _), (_, _, d2, _)) if d1 <= d2 => (ray_h, false),
            _ => (ray_v, true),
        };
        draw_ray(canvas, player, hit, Color::WHITE);
        let (_, _, distance, tile) = hit;

        let adj_distance = distance * offset.cos();
        let ray_height = TILE_SIZE * n_rays as f64 / adj_distance;
        let tex_x = ray_to_tex_coordinatinates(hit.0, hit.1, horiz);
        hits.push(RayHit {
            height: min(height, ray_height as u32),
            tile,
            horizontal: horiz,
            tex_x,
        });
    }
    hits
}

fn draw_ray<T: RenderTarget>(
    canvas: &mut Canvas<T>,
    player: &Player,
    ray: (f64, f64, f64, u16),
    color: Color,
) {
    let (x, y, _, _) = ray;
    canvas.set_draw_color(color);
    canvas
        .draw_line(
            Point::new(player.x.round() as i32, player.y.round() as i32),
            Point::new(x as i32, y as i32),
        )
        .unwrap();
}

//canvas parameter left here to facilitate debug drawings
fn cast_ray_v<T: RenderTarget>(
    map: &Map,
    _canvas: &mut Canvas<T>,
    player: &Player,
    ray_offset: f64,
) -> (f64, f64, f64, u16) {
    let ray_angle = norm_angle(player.view_angle + ray_offset);

    //looking to the side -- cannot hit a horizontal line
    if ray_angle == ANGLE_LEFT || ray_angle == ANGLE_RIGHT {
        return (0.0, 0.0, f64::INFINITY, 0);
    }

    let (rx, ry, xo, yo) = if !(ANGLE_RIGHT..=ANGLE_LEFT).contains(&ray_angle) {
        let round_y = ctrunc(player.y, MAP_SCALE_H, 1.0);
        let a = round_y - player.y;
        let b = a * ray_angle.tan();
        let c = MAP_SCALE_H as f64 * ray_angle.tan();
        (player.x + b, round_y, c, MAP_SCALE_H as f64)
    } else {
        let round_y = ctrunc(player.y, MAP_SCALE_H, 0.0);
        let a = player.y - round_y;
        let b = a * ray_angle.tan();
        let c = MAP_SCALE_H as f64 * ray_angle.tan();
        (
            player.x - b,
            round_y - 0.000001,
            -1.0 * c,
            -1.0 * MAP_SCALE_H as f64,
        )
    };
    follow_ray(map, player, rx, ry, xo, yo)
}

fn cast_ray_h<T: RenderTarget>(
    map: &Map,
    _canvas: &mut Canvas<T>,
    player: &Player,
    ray_offset: f64,
) -> (f64, f64, f64, u16) {
    let ray_angle = norm_angle(player.view_angle + ray_offset);

    //looking up/down -- cannot hit a vertical line
    if ray_angle == ANGLE_UP || ray_angle == ANGLE_DOWN {
        return (0.0, 0.0, f64::INFINITY, 0);
    }

    let (rx, ry, xo, yo) = if ray_angle < ANGLE_UP {
        // looking right -- increasing x
        let round_x = ctrunc(player.x, MAP_SCALE_W, 1.0);
        let b = round_x - player.x;
        let a = b / ray_angle.tan();
        let c = MAP_SCALE_W as f64 / ray_angle.tan();
        (round_x, player.y + a, MAP_SCALE_W as f64, c)
    } else {
        let round_x = ctrunc(player.x, MAP_SCALE_W, 0.0);
        let b = player.x - round_x;
        let a = b / ray_angle.tan();
        let c = MAP_SCALE_W as f64 / ray_angle.tan();
        (
            round_x - 0.00001,
            player.y - a,
            -1.0 * MAP_SCALE_W as f64,
            -1.0 * c,
        )
    };
    follow_ray(map, player, rx, ry, xo, yo)
}

fn follow_ray(
    map: &Map,
    player: &Player,
    x: f64,
    y: f64,
    xo: f64,
    yo: f64,
) -> (f64, f64, f64, u16) {
    let (mut rx, mut ry) = (x, y);
    for _ in 1..MAP_HEIGHT {
        match read_map(map, rx, ry) {
            Ok(Tile::Wall(tile)) => {
                return (rx, ry, distance(player, rx, ry), tile);
            }
            Err(_) => {
                return (rx, ry, distance(player, rx, ry), 0);
            }
            _ => {}
        }
        rx += xo;
        ry += yo;
    }

    (rx, ry, distance(player, rx, ry), 0)
}

fn read_map(map: &Map, x: f64, y: f64) -> Result<Tile, Nothing> {
    let mx = cdiv(x, MAP_SCALE_W, 0.0);
    let my = cdiv(y, MAP_SCALE_H, 0.0);
    if mx >= MAP_WIDTH || my >= MAP_HEIGHT {
        Err(Nothing)
    } else {
        Ok(map.tile_at(mx as u8, my as u8))
    }
}

/// Turn the ray hit (x, y) coordinates in to the x-coordinate within the texture.
/// This is obtained by translating the coords first to the tilemap dimensions,
/// and, since each integer represents a tile, the fractional part determines what
/// part of the texture the ray hit.
// TODO consider moving this over to the drawing routine instead
fn ray_to_tex_coordinatinates(rx: f64, ry: f64, horizontal: bool) -> usize {
    let tx = (rx / MAP_SCALE_W as f64).fract();
    let ty = (ry / MAP_SCALE_H as f64).fract();

    let fract = if horizontal {
        if ty < 0.5 {
            1.0 - tx
        } else {
            tx
        }
    } else if tx < 0.5 {
        ty
    } else {
        1.0 - ty
    };
    (fract * WALLPIC_WIDTH as f64) as usize
}

fn cdiv(x: f64, scale: u32, updown: f64) -> usize {
    (x / scale as f64 + updown).trunc() as usize
}

fn ctrunc(x: f64, scale: u32, updown: f64) -> f64 {
    (x / scale as f64 + updown).trunc() * scale as f64
}

fn distance(player: &Player, x: f64, y: f64) -> f64 {
    (pow(x - player.x, 2) + pow(y - player.y, 2)).sqrt()
}
