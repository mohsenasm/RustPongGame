use std::ops::{Add, AddAssign};

use super::terminal::*;

#[derive(Clone, Copy)]
pub struct XY {
    pub x: isize,
    pub y: isize,
}

impl Add for XY {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r: XY = XY { x: 0, y: 0 };
        r.x = self.x + other.x;
        r.y = self.y + other.y;
        r
    }
}

impl AddAssign for XY {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

pub struct Board {
    pub inner_size: XY,
    pub ball_position: XY,
    pub ball_direction: XY,
    pub plate_length: isize,
    pub plate_position_x: isize,
}

const PLATE_STEP: isize = 3;
const BALL_X_SIZE: isize = 2;

impl Board {
    pub fn update(&mut self, input_command: Option<String>) -> bool {
        let mut is_game_over = false;

        // parse a command
        match input_command {
            Some(command) => {
                if command == "left" {
                    if self.plate_position_x < PLATE_STEP {
                        self.plate_position_x = 0;
                    } else {
                        self.plate_position_x = self.plate_position_x - PLATE_STEP
                    }
                } else if command == "right" {
                    if self.plate_position_x + self.plate_length + PLATE_STEP >= self.inner_size.x {
                        self.plate_position_x = self.inner_size.x - self.plate_length;
                    } else {
                        self.plate_position_x = self.plate_position_x + PLATE_STEP
                    }
                }
            }
            None => (),
        }
        // update ball position & direction
        self.ball_position += self.ball_direction;
        if self.ball_position.y + 1 > self.inner_size.y {
            self.ball_position.y = self.inner_size.y - 1;
            self.ball_direction.y *= -1;

            if (self.ball_position.x + BALL_X_SIZE < self.plate_position_x + 1)
                || (self.ball_position.x > self.plate_position_x + self.plate_length - 1)
            {
                is_game_over = true;
            }
        }
        if self.ball_position.x + BALL_X_SIZE > self.inner_size.x {
            self.ball_position.x = self.inner_size.x - BALL_X_SIZE;
            self.ball_direction.x *= -1;
        }
        if self.ball_position.y < 0 {
            self.ball_position.y = 0;
            self.ball_direction.y *= -1;
        }
        if self.ball_position.x < 0 {
            self.ball_position.x = 0;
            self.ball_direction.x *= -1;
        }

        is_game_over
    }
    pub fn validate(&self) {
        assert!(self.inner_size.x >= self.plate_position_x + self.plate_length);
        assert!(
            self.inner_size.x >= self.ball_position.x + BALL_X_SIZE && self.ball_position.x >= 0
        );
        assert!(self.inner_size.y > self.ball_position.y && self.ball_position.y >= 0);
    }
    pub fn draw(&self) {
        self.validate();
        clear_terminal();
        // top
        println!(
            "{}{}{}\r",
            JOINT_TOP_LEFT,
            LINE_HORIZONTAL.repeat(self.inner_size.x),
            JOINT_TOP_RIGHT
        );
        // middle
        for row in 0..self.inner_size.y {
            if self.ball_position.y == row {
                println!(
                    "{}{}{}{}{}\r",
                    LINE_VERTICAL,
                    SPACE.repeat(self.ball_position.x),
                    BALL,
                    SPACE.repeat(self.inner_size.x - self.ball_position.x - BALL_X_SIZE),
                    LINE_VERTICAL
                );
            } else {
                println!(
                    "{}{}{}\r",
                    LINE_VERTICAL,
                    SPACE.repeat(self.inner_size.x),
                    LINE_VERTICAL
                );
            }
        }
        // bottom
        println!(
            "{}{}{}{}{}\r",
            JOINT_BOTTOM_LEFT,
            SPACE.repeat(self.plate_position_x),
            LINE_HORIZONTAL.repeat(self.plate_length),
            SPACE.repeat(self.inner_size.x - self.plate_length - self.plate_position_x),
            JOINT_BOTTOM_RIGHT
        );
    }
}
