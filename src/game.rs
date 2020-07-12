use ggez::nalgebra::{Point2, Vector2};

use crate::gfx::{get_sprite, Sprite, SpriteType};
use crate::level::Level;

pub enum ControlType {
    Right,
    Left,
    Up,
    Down,
}

pub struct Control {
    pub energy: u32,
    pub control_type: ControlType,
}

impl Control {
    pub fn activate(&mut self, player: &mut Player, level: &mut Level) {
        assert!(self.energy > 0);
        self.energy -= 1;
        match self.control_type {
            ControlType::Right => player.walk(level, Vector2::new(1, 0)),
            ControlType::Left => player.walk(level, Vector2::new(-1, 0)),
            ControlType::Up => player.walk(level, Vector2::new(0, -1)),
            ControlType::Down => player.walk(level, Vector2::new(0, 1)),
        };
    }

    pub fn has_energy(&self) -> bool {
        self.energy > 0
    }
}

pub struct Player {
    pub pos: Point2<usize>,
    pub sprite: Sprite,
    pub pending_items: Vec<ItemType>,
}

impl Player {
    fn walk(&mut self, level: &mut Level, delta: Vector2<isize>) {
        let x = (self.pos.x as isize + delta.x) as usize;
        let y = (self.pos.y as isize + delta.y) as usize;
        let new_pos = match level.get(x, y).unwrap().tile_type {
            TileType::Wall => self.pos,
            TileType::Floor | TileType::Exit => Point2::new(x, y),
        };
        self.pos = new_pos;
        if let Some(item) = level.items.remove(&(x, y)) {
            self.pending_items.push(item.item_type);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub sprite: Sprite,
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        let sprite_type = match tile_type {
            TileType::Floor => SpriteType::Floor,
            TileType::Wall => SpriteType::Wall,
            TileType::Exit => SpriteType::Exit,
        };
        Tile {
            sprite: get_sprite(sprite_type),
            tile_type,
        }
    }
}

pub struct Item {
    pub sprite: Sprite,
    pub item_type: ItemType,
}

pub enum ItemType {
    DownControl,
    LeftControl,
    UpControl,
    RightControl,
}

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        let sprite_type = match item_type {
            ItemType::UpControl => SpriteType::UpControl,
            ItemType::RightControl => SpriteType::RightControl,
            ItemType::DownControl => SpriteType::DownControl,
            ItemType::LeftControl => SpriteType::LeftControl,
        };
        Item {
            sprite: get_sprite(sprite_type),
            item_type,
        }
    }
}
