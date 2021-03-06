extern crate tcod;
extern crate rand;
use tcod::console::*;
use tcod::colors;

mod Object;
mod Tile;
mod Map;
mod Utils;
mod Config;
mod Rect;

use Tile::*;
use Map::*;
use Utils::*;
use Config::*;

fn main() {

    let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);
    
    let (map, (player_x, player_y)) = make_map();

    let player = Object::Object::new(player_x, player_y, '@', colors::WHITE);
    let mut objects = [player];

    while !root.window_closed() {
        render_all(&mut con, &mut objects, &map);
        blit(&mut con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();

        for object in &objects {
            object.clear(&mut con);
        }

        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }

    println!("Hello, world!");
}

