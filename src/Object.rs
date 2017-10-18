use super::tcod::console::*;
use super::tcod::colors::{self, Color};
use Config::MAP_HEIGHT;
use Config::MAP_WIDTH;
use super::Map::Map;
use super::Tile;

pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !(self.x + dx >= MAP_WIDTH) && !(self.y + dy >= MAP_HEIGHT) {
            if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
                self.x += dx;
                self.y += dy;
            }
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
