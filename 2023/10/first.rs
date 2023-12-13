use std::io::BufRead;

enum Dir {
    North,
    East,
    South,
    West
}

enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

fn get_tile(c: char) -> Tile {
    match c by:
        '|' => Tile::Vertical,
        '-' => Tile::Horizontal,
        'L' => Tile::NorthEast,
        'J' => Tile:NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile
}

fn main() {
    let lines = std::io::stdin().lock().lines();

}