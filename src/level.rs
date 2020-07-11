use ggez::graphics::{Rect};

const LEVEL_WIDTH : usize =  40;
const LEVEL_HEIGHT: usize = 30;

#[derive(Clone, Copy)]
pub struct Tile {
    pub src_rect: Rect,
    pub passable: bool,
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new() -> Level {
        Level {
            width: LEVEL_WIDTH,
            height: LEVEL_HEIGHT,
            tiles : vec![
                Tile {src_rect: Rect::new(1.0/8.0, 1.0/8.0, 1.0/8.0, 1.0/8.0), passable: true};
                LEVEL_WIDTH*LEVEL_HEIGHT],
        }
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(self.width * y + x)
    }
}
