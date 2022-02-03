use std::ops::BitAnd;

use sdl2::{event::Event, keyboard::Keycode, Sdl};

// typedef	struct		{
//     ScanCode	button0,button1,
//                 upleft,		up,		upright,
//                 left,				right,
//                 downleft,	down,	downright;
// } KeyboardDef;
pub struct KeyboardDef {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left,
    Up,
    None,
    Right,
    Down,
}

impl Motion {
    pub fn value(&self) -> i16 {
        match self {
            Motion::Left | Motion::Up => -1,
            Motion::None => 0,
            Motion::Right | Motion::Down => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ControlInfo {
    button0: bool,
    button1: bool,
    button2: bool,
    button3: bool,
    x: i16,
    y: i16,
    xaxis: Motion,
    yaxis: Motion,
    dir: Direction,
}

impl Default for ControlInfo {
    fn default() -> Self {
        ControlInfo {
            button0: false,
            button1: false,
            button2: false,
            button3: false,
            x: 0,
            y: 0,
            xaxis: Motion::None,
            yaxis: Motion::None,
            dir: Direction::None,
        }
    }
}

pub struct InputManager<'a> {
    sdl_context: &'a Sdl,
    last_scan: Option<Keycode>,
    current_key: Option<Keycode>,
    should_exit: bool,
}

impl<'a> InputManager<'a> {
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn wait_for_key(&self) {
        'running: loop {
            for event in self.sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { .. } => break 'running,
                    _ => {}
                }
            }
        }
    }
    pub fn process_events(&mut self) {
        for event in self.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.should_exit = true;
                }
                Event::KeyDown { keycode, .. } => {
                    // check for keypresses
                    if keycode == Some(Keycode::ScrollLock) || keycode == Some(Keycode::F12) {
                        // set window grab
                    }
                    dbg!(keycode);
                    self.last_scan = keycode;
                    let mod_state = self.sdl_context.keyboard().mod_state();
                    dbg!(mod_state);

                    // close with Alt + F4
                    if let Some(Keycode::LAlt) = self.current_key {
                        if let Some(Keycode::F4) = self.last_scan {
                            panic!();
                        }
                    }

                    match self.last_scan {
                        Some(Keycode::KpEnter) => self.last_scan = Some(Keycode::Return),
                        Some(Keycode::RShift) => self.last_scan = Some(Keycode::LShift),
                        Some(Keycode::RAlt) => self.last_scan = Some(Keycode::LAlt),
                        Some(Keycode::RCtrl) => self.last_scan = Some(Keycode::LCtrl),
                        _ => {
                            if mod_state.bitand(sdl2::keyboard::Mod::NUMMOD).bits() != 0 {
                                match self.last_scan {
                                    Some(Keycode::Kp2) => self.last_scan = Some(Keycode::Down),
                                    Some(Keycode::Kp4) => self.last_scan = Some(Keycode::Left),
                                    Some(Keycode::Kp6) => self.last_scan = Some(Keycode::Right),
                                    Some(Keycode::Kp8) => self.last_scan = Some(Keycode::Up),
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                    // TODO:
                    //             int sym = LastScan;
                    //             if(sym >= 'a' && sym <= 'z')
                    //                 sym -= 32;  // convert to uppercase
                    //             if(mod & (KMOD_SHIFT | KMOD_CAPS))
                    //             {
                    //                 if(sym < lengthof(ShiftNames) && ShiftNames[sym])
                    //                     LastASCII = ShiftNames[sym];
                    //             }
                    //             else
                    //             {
                    //                 if(sym < lengthof(ASCIINames) && ASCIINames[sym])
                    //                     LastASCII = ASCIINames[sym];
                    //             }
                    // //            if(LastScan<SDLK_LAST)
                    //             Keyboard[LastScan] = 1;
                    //             if(LastScan == SDLK_PAUSE)
                    //                 Paused = true;
                    //             break;
                    self.current_key = self.last_scan
                }
                Event::KeyUp { .. } => {
                    // todo
                }
                Event::Window { .. } => { // check if this is the same as SDL_WINDOWEVENT
                     // todo
                }
                _ => {}
            }
        }
    }

    /// Starts up the Input Manager
    pub fn startup(sdl_context: &Sdl) -> InputManager {
        InputManager {
            sdl_context,
            last_scan: None,
            current_key: None,
            should_exit: false,
        }
    }

    /// Reads the device associated with the specified player and fills in the control info struct
    pub fn read_control(&mut self, info: &mut ControlInfo) {
        // maybe info can be removed and return a new instance each time?
        let mut buttons: u16 = 0;
        let dx: i16;
        let dy: i16;
        let mut mx = Motion::None;
        let mut my = Motion::None;

        self.process_events();
        if let Some(keycode) = self.current_key {
            match keycode {
                Keycode::Left => mx = Motion::Left,
                Keycode::Right => mx = Motion::Right,
                Keycode::Up => my = Motion::Up,
                Keycode::Down => my = Motion::Down,
                Keycode::LCtrl | Keycode::RCtrl => {
                    // button0
                    buttons += 1 << 0;
                }
                Keycode::RAlt | Keycode::LAlt => {
                    // button1
                    buttons += 1 << 1;
                }
                Keycode::PageDown => {
                    // downright
                    mx = Motion::Right;
                    my = Motion::Down;
                }
                Keycode::PageUp => {
                    // upright
                    mx = Motion::Right;
                    my = Motion::Up;
                }
                Keycode::End => {
                    // downleft
                    mx = Motion::Left;
                    my = Motion::Down;
                }
                Keycode::Home => {
                    // upleft
                    mx = Motion::Left;
                    my = Motion::Up;
                }
                _ => {}
            }

            dx = mx.value() * 127;
            dy = my.value() * 127;

            let mut previous_info = info.clone();

            info.x = dx;
            info.y = dy;
            info.yaxis = my;
            info.button0 = (buttons & (1 << 0)) != 0;
            info.button1 = (buttons & (1 << 1)) != 0;
            info.button2 = (buttons & (1 << 2)) != 0;
            info.button3 = (buttons & (1 << 3)) != 0;
            info.dir = Direction::from(((my.value() + 1) * 3) + (mx.value() + 1));

            if &mut previous_info != info {
                dbg!(info);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    None,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    pub fn from(number: i16) -> Direction {
        match number {
            0 => Direction::NorthWest,
            1 => Direction::North,
            2 => Direction::NorthEast,
            3 => Direction::West,
            4 => Direction::None,
            5 => Direction::East,
            6 => Direction::SouthWest,
            7 => Direction::South,
            8 => Direction::SouthEast,
            _ => unreachable!(),
        }
    }
}
