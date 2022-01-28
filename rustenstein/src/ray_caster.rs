extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;
use std::f64::consts::PI;
use num::pow;

mod map_data;

use map_data::Tile;
use map_data::tile_at;

const WIDTH_2D:u32 =  1536;
const HEIGHT_2D:u32 = 1536;
const MAP_H:u32 = 64;
const MAP_W:u32 = 64;
const MAP_SCALE_H:u32 = HEIGHT_2D / MAP_H;
const MAP_SCALE_W:u32 = WIDTH_2D / MAP_W;
const PLAYER_DIAM:i32 = 6;
const PLAYER_LEN:f64 = 40.0;
const ROTATE_SPEED:f64 = 0.02;
const MOVE_SPEED:f64 = 2.5;
const FIELD_OF_VIEW:f64 = PI/2.0;
const N_RAYS:u32 = 320;
const ANGLE_DOWN:f64 = 0.0;
const ANGLE_UP:f64 = PI;
const ANGLE_LEFT:f64 = 3.0*PI/2.0;
const ANGLE_RIGHT:f64 = PI/2.0;

struct Player {
    x: f64,
    y: f64,
    angle: f64
}

struct Nothing;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_2d = video_subsystem.window("", WIDTH_2D, HEIGHT_2D)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas_2d = window_2d.into_canvas().build().unwrap();
    canvas_2d.set_draw_color(Color::RGB(0, 255, 255));
    canvas_2d.clear();
    canvas_2d.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut player = Player { x: WIDTH_2D as f64 / 3.0,
                              y: HEIGHT_2D as f64 / 3.0,
                              angle: 0.0 };
    let mut left_down = false;
    let mut right_down = false;
    let mut up_down = false;
    let mut down_down = false;
    'main: loop {
        canvas_2d.set_draw_color(Color::RGB(64, 64, 64));
        canvas_2d.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    left_down = true
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    left_down = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    right_down = true
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    right_down = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    up_down = true
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    up_down = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    down_down = true
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    down_down = false;
                },
                _ => {}
            }
        }
        if left_down {
            player.angle += ROTATE_SPEED;
        }
        if right_down {
            player.angle -= ROTATE_SPEED;
        }
        player.angle = norm_angle(player.angle);
        if up_down {
            player.x += player.angle.sin() * MOVE_SPEED;
            player.y += player.angle.cos() * MOVE_SPEED;
        }
        if down_down {
            player.x -= player.angle.sin() * MOVE_SPEED;
            player.y -= player.angle.cos() * MOVE_SPEED;
        }
        // The rest of the game loop goes here...
        draw_map(&mut canvas_2d);
        draw_rays(&mut canvas_2d, &mut player);
        draw_player(&mut canvas_2d, &mut player);
        canvas_2d.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_map<T: RenderTarget>(canvas: &mut Canvas<T>) {
    for y in 0..(MAP_H) {
        for x in 0..(MAP_W) {
            //let i = (y * MAP_W + x) as usize;
            let color =
                match tile_at(x as u8, y as u8) {
                    Tile::Wall(_) => Color::RGB(64,64,255),
                    _ => Color::RGB(0,0,0)
                };
            canvas.set_draw_color(color);
            canvas.fill_rect(
                Rect::new((MAP_SCALE_W * x + 1).try_into().unwrap(),
                          (MAP_SCALE_H * y + 1).try_into().unwrap(),
                          (MAP_SCALE_W - 1).try_into().unwrap(),
                          (MAP_SCALE_H - 1).try_into().unwrap()
                )).unwrap();
        }
    }
}

fn draw_player<T: RenderTarget>(canvas: &mut Canvas<T>, player: &mut Player) {
    canvas.set_draw_color(Color::RGB(255,255,0));
    let x = player.x.round() as i32;
    let y = player.y.round() as i32;
    let nx = (player.x + player.angle.sin() * PLAYER_LEN).round() as i32;
    let ny = (player.y + player.angle.cos() * PLAYER_LEN).round() as i32;
    canvas.fill_rect(Rect::new( x - PLAYER_DIAM, y - PLAYER_DIAM,
                                PLAYER_DIAM as u32 * 2, PLAYER_DIAM as u32 * 2 )).unwrap();
    canvas.draw_line(Point::new( x, y ),
                     Point::new( nx, ny )).unwrap();
}


fn draw_rays<T: RenderTarget>(canvas: &mut Canvas<T>, player: &mut Player) {
    let step_angle = FIELD_OF_VIEW / (N_RAYS as f64);
    for i in 0..N_RAYS {
        let offset = (i as f64)*step_angle - FIELD_OF_VIEW / 2.0;
        let ray_h = cast_ray_h(canvas, player, offset);
        let ray_v = cast_ray_v(canvas, player, offset);
        let hit = match (ray_h, ray_v) {
            ((_,_,d1,_), (_,_,d2,_)) if d1 <= d2 => { ray_h },
            _ => { ray_v }
        };
        draw_ray(canvas, player, hit, Color::WHITE);
    }
}

fn draw_ray<T: RenderTarget>(canvas: &mut Canvas<T>,
                             player: &mut Player,
                             ray: (f64,f64,f64,u32),
                             color: Color) {
    let (x,y,_,_) = ray;
    canvas.set_draw_color(color);
    canvas.draw_line(Point::new( player.x.round() as i32,
                                 player.y.round() as i32 ),
                     Point::new( x as i32,
                                 y as i32)

    ).unwrap();
}

//canvas parameter left here to facilitate debug drawings
fn cast_ray_v<T: RenderTarget>(_canvas: &mut Canvas<T>,
                               player: &mut Player,
                               ray_offset: f64) -> (f64, f64, f64, u32) {
    let ray_angle = norm_angle(player.angle + ray_offset);

    //looking to the side -- cannot hit a horizontal line
    if ray_angle == ANGLE_LEFT || ray_angle == ANGLE_RIGHT {
        return (0.0,0.0,f64::INFINITY,0);
    }

    let (mut rx,mut ry,xo,yo) =
        if ray_angle > ANGLE_LEFT || ray_angle < ANGLE_RIGHT {
        let round_y = ctrunc(player.y, MAP_SCALE_H, 1.0);
        let a = round_y - player.y;
        let b = a * ray_angle.tan();
        let c = MAP_SCALE_H as f64 * ray_angle.tan();
        (player.x + b, round_y,
         c, MAP_SCALE_H as f64)
    } else {
        let round_y = ctrunc(player.y, MAP_SCALE_H, 0.0);
        let a = player.y - round_y;
        let b = a * ray_angle.tan();
        let c = MAP_SCALE_H as f64 * ray_angle.tan();
        (player.x - b, round_y - 0.000001,
         -1.0 * c, -1.0 * MAP_SCALE_H as f64)
    };
    return follow_ray(player,rx,ry,xo,yo);
}

fn cast_ray_h<T: RenderTarget>(_canvas: &mut Canvas<T>,
                               player: &mut Player,
                               ray_offset: f64) -> (f64, f64, f64, u32) {
    let ray_angle = norm_angle(player.angle + ray_offset);

    //looking up/down -- cannot hit a vertical line
    if ray_angle == ANGLE_UP || ray_angle == ANGLE_DOWN {
        return (0.0,0.0,f64::INFINITY,0);
    }

    let (mut rx,mut ry,xo,yo) =
    if ray_angle < ANGLE_UP { // looking right -- increasing x
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
        (round_x - 0.00001, player.y - a,
         -1.0 * MAP_SCALE_W as f64, -1.0 * c)
    };
    return follow_ray(player,rx,ry,xo,yo);
}

fn follow_ray(player: &mut Player, x:f64, y:f64, xo:f64, yo:f64) -> (f64, f64, f64, u32) {
    let (mut rx, mut ry) = (x,y);
    for _ in 1..MAP_H {
        match read_map(rx, ry) {
            Ok(Tile::Wall(_)) => {
                return (rx, ry, distance(player, rx,ry), 1);
            },
            Err(_) => {
                return (rx, ry, distance(player, rx,ry), 0);
            },
            _ => {}
        }
        rx += xo;
        ry += yo;
    }

    return (rx, ry, distance(player,rx,ry), 0);
}

fn read_map(x:f64, y:f64) -> Result<Tile, Nothing> {
    let mx = cdiv(x, MAP_SCALE_W, 0.0);
    let my = cdiv(y, MAP_SCALE_H, 0.0);
    if mx >= MAP_W as usize || my >= MAP_H as usize {
        Err(Nothing)
    } else {
        Ok(tile_at(mx as u8, my as u8))
    }
}

fn cdiv(x:f64, scale:u32, updown:f64) -> usize {
    (x / scale as f64 + updown).trunc() as usize
}

fn ctrunc(x:f64, scale:u32, updown:f64) -> f64 {
    (x / scale as f64 + updown).trunc() * scale as f64
}

fn norm_angle(a:f64) -> f64 {
    let nrots = (a / (2.0 * PI)).trunc() - if a < 0.0 { 1.0 } else { 0.0 };
    a - nrots * 2.0 * PI
}

fn distance(player: &mut Player, x: f64, y: f64) -> f64 {
    (pow(x-player.x,2) + pow(y-player.y,2)).sqrt()
}
