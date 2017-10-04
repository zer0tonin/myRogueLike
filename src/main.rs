extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

fn main() {
    const SCREEN_WIDTH: i32 = 80;
    const SCREEN_HEIGHT: i32 = 50;
    const LIMIT_FPS: i32 = 20;

    let mut root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        root.flush();

        root.put_char(player_x, player_y, ' ', BackgroundFlag::None);
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }

    println!("Hello, world!");
}

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,

        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

        _ => {},
    }
    false
}
