use ggez::filesystem;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::gfx::{get_sprite, Sprite, SpriteType};

const LEVEL_WIDTH: usize = 40;
const LEVEL_HEIGHT: usize = 30;

#[derive(Clone, Copy)]
pub struct Tile {
    pub sprite: Sprite,
    pub passable: bool,
}

impl Tile {
    fn new(passable: bool) -> Tile {
        let sprite_type = if passable {
            SpriteType::Floor
        } else {
            SpriteType::Wall
        };
        Tile {
            sprite: get_sprite(sprite_type),
            passable,
        }
    }
}

pub struct Item {
    pub sprite: Sprite,
}

enum ItemType
{
    DownControl,
    UpControl,
}

impl Item {
    fn new(item_type: ItemType) -> Item {
        let sprite_type = match item_type {
            ItemType::DownControl => SpriteType::DownControl,
            ItemType::UpControl => SpriteType::UpControl,
        };
        Item {
            sprite: get_sprite(sprite_type)
        }
    }
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub player_start: Point2<usize>,
    tiles: Vec<Tile>,
    pub items: HashMap<(usize, usize), Item>,
}

impl Level {
    pub fn new() -> Level {
        Level {
            width: LEVEL_WIDTH,
            height: LEVEL_HEIGHT,
            player_start: Point2::new(0, 0),
            tiles: vec![Tile::new(true); LEVEL_WIDTH * LEVEL_HEIGHT],
            items: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(ctx: &mut Context, filename: P) -> GameResult<Level> {
        let file = filesystem::open(ctx, filename)?;
        let reader = BufReader::new(file);
        let mut level = Level::new();
        for (row, line) in reader.lines().enumerate() {
            for (col, c) in line.unwrap().chars().enumerate() {
                level.set_from_char(col, row, c);
            }
        }
        Ok(level)
    }

    fn set_from_char(&mut self, col: usize, row: usize, c: char) {
        let tile = self.get_mut(col, row).unwrap();
        *tile = match c {
            '#' => Tile::new(false),
            //'>' => Tile::new(Downstairs),
            _ => Tile::new(true),
        };
        match c {
            '@' => {
                self.player_start = Point2::new(col, row);
            },
            'a' => {
                self.items.insert((col, row), Item::new(ItemType::DownControl));
            },
            'b' => {
                self.items.insert((col, row), Item::new(ItemType::UpControl));
            },
            _ => {},
        }
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(self.width * y + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(self.width * y + x)
    }
}
