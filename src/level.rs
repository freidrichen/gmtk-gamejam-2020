use ggez::graphics::Rect;
use ggez::filesystem;
use ggez::{Context, GameResult};
use std::path::Path;
use std::io::{BufRead, BufReader};

const LEVEL_WIDTH : usize =  40;
const LEVEL_HEIGHT: usize = 30;
const NUM_SPRITES_X: usize = 8;
const NUM_SPRITES_Y: usize = 8;

fn sprite(index: usize) -> Rect {
    let row = index / NUM_SPRITES_X;
    let col = index % NUM_SPRITES_X;
    assert!(row < NUM_SPRITES_Y);
    Rect::new(
        row as f32 / NUM_SPRITES_X as f32,
        col as f32 / NUM_SPRITES_Y as f32,
        1.0 / NUM_SPRITES_X as f32,
        1.0 / NUM_SPRITES_Y as f32,
    )
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub src_rect: Rect,
    pub passable: bool,
}

impl Tile {
    fn new(passable: bool) -> Tile {
        let sprite_index: usize = passable as usize;
        Tile {
            src_rect: sprite(sprite_index),
            passable,
        }
    }
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
                Tile::new(true);
                LEVEL_WIDTH*LEVEL_HEIGHT],
        }
    }

    pub fn load<P: AsRef<Path>>(ctx: &mut Context, filename: P) -> GameResult<Level> {
        let file = filesystem::open(ctx, filename)?;
        let reader = BufReader::new(file);
        let mut level = Level::new();
        for (row, line) in reader.lines().enumerate() {
            for (col, c) in line.unwrap().chars().enumerate() {
                if let Some(tile) = level.get_mut(col, row) {
                    *tile = Tile::new(c == '#')
                }
            }
        }
        Ok(level)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(self.width * y + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(self.width * y + x)
    }
}
