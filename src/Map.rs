use super::Tile::*;
use Config::MAP_HEIGHT;
use Config::MAP_WIDTH;

pub type Map = Vec<Vec<Tile>>;

pub fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}
