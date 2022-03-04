use crate::constants;

const ROTATE_SPEED: f64 = 0.02;
const MOVE_SPEED: f64 = 2.5;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Player {
    pub fn move_forward(&mut self) {
        self.x += self.angle.sin() * MOVE_SPEED;
        self.y += self.angle.cos() * MOVE_SPEED;
    }

    pub fn move_backward(&mut self) {
        self.x -= self.angle.sin() * MOVE_SPEED;
        self.y -= self.angle.cos() * MOVE_SPEED;
    }

    pub fn turn_left(&mut self) {
        self.angle += ROTATE_SPEED;
        self.angle = constants::norm_angle(self.angle);
    }

    pub fn turn_right(&mut self) {
        self.angle -= ROTATE_SPEED;
        self.angle = constants::norm_angle(self.angle);
    }
}
