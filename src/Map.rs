use Tile::*;
use Config::MAP_HEIGHT;
use Config::MAP_WIDTH;
use Rect::*;

pub type Map = Vec<Vec<Tile>>;

pub fn make_map() -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 15);
    create_room(room1, &mut map);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room2, &mut map);
    
    create_h_tunnel(25, 55, 23, &mut map);

    map
}
