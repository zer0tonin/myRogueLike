extern crate tcod;
use tcod::console::*;
use tcod::colors;

mod Object;
mod Tile;
mod Map;
mod Utils;
mod Config;

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
    
    let player = Object::Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let mut objects = [player, npc];

    let mut map = make_map();

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

