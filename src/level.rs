use ggez::filesystem;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use crate::game::{Control, ControlType, Item, ItemType, Tile, TileType};

pub const LEVEL_WIDTH: usize = 20;
pub const LEVEL_HEIGHT: usize = 15;

pub struct Level {
    pub number: usize,
    pub width: usize,
    pub height: usize,
    pub player_start: Point2<usize>,
    tiles: Vec<Tile>,
    pub items: HashMap<(usize, usize), Item>,
    pub controls_start: [Option<Control>; 4],
}

impl Level {
    pub fn new() -> Level {
        Level {
            number: 0,
            width: LEVEL_WIDTH,
            height: LEVEL_HEIGHT,
            player_start: Point2::new(0, 0),
            tiles: vec![Tile::new(TileType::Floor); LEVEL_WIDTH * LEVEL_HEIGHT],
            items: HashMap::new(),
            controls_start: [None, None, None, None],
        }
    }

    pub fn load(ctx: &mut Context, number: usize) -> GameResult<Level> {
        let file = filesystem::open(ctx, format!("/level{}.txt", number))?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut level = Level::new();
        for (row, line) in lines.by_ref().take(LEVEL_HEIGHT).enumerate() {
            for (col, c) in line.unwrap().chars().enumerate() {
                level.set_from_char(col, row, c);
            }
        }
        for (i, line) in lines.by_ref().take(4).enumerate() {
            let line = line.unwrap();
            level.controls_start[i] = if line.len() > 0 {
                let parts: Vec<_> = line.split(" ").collect();
                let control_type = match parts[0] {
                    "<" => ControlType::Left,
                    ">" => ControlType::Right,
                    "v" => ControlType::Down,
                    "^" => ControlType::Up,
                    _ => panic!(),
                };
                let energy: u32 = parts[1].parse().unwrap();
                Some(Control {
                    control_type,
                    energy,
                })
            } else {
                None
            };
        }
        Ok(level)
    }

    fn set_from_char(&mut self, col: usize, row: usize, c: char) {
        let tile = self.get_mut(col, row).unwrap();
        *tile = match c {
            '#' => Tile::new(TileType::Wall),
            '>' => Tile::new(TileType::Exit),
            _ => Tile::new(TileType::Floor),
        };
        match c {
            '@' => {
                self.player_start = Point2::new(col, row);
            }
            'u' => {
                self.items
                    .insert((col, row), Item::new(ItemType::UpControl));
            }
            'r' => {
                self.items
                    .insert((col, row), Item::new(ItemType::RightControl));
            }
            'd' => {
                self.items
                    .insert((col, row), Item::new(ItemType::DownControl));
            }
            'l' => {
                self.items
                    .insert((col, row), Item::new(ItemType::LeftControl));
            }
            _ => {}
        }
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(self.width * y + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(self.width * y + x)
    }
}
