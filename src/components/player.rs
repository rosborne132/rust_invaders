use super::constants::{NUM_COLS, NUM_ROWS};
use std::time::Duration;

use super::frame::{Drawable, Frame};
use super::shot::Shot;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        // This line controls how many shots we can have out
        // at a time.
        if self.shots.len() < 20 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            return true;
        } else {
            return false;
        }
    }

    pub fn update (&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";

        // Draw shots
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
