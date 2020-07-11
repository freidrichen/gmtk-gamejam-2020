use ggez::graphics::Rect;
use ggez::nalgebra::Point2;

const NUM_SPRITES_X: usize = 8;
const NUM_SPRITES_Y: usize = 8;
const SPRITE_WIDTH: usize = 8;
const SPRITE_HEIGHT: usize = 8;
pub const SPRITE_SCALE: f32 = 2.0;

pub type Sprite = Rect;

pub enum SpriteType {
    Player,
    Wall,
    Floor,
    Exit,
    DownControl,
    UpControl,
}

pub fn screen_pos(tile_pos: Point2<usize>) -> Point2<f32> {
    Point2::new(
        (tile_pos.x * SPRITE_WIDTH) as f32,
        (tile_pos.y * SPRITE_HEIGHT) as f32,
    )
}

pub fn get_sprite(sprite_type: SpriteType) -> Rect {
    let index = match sprite_type {
        SpriteType::Player => 8,
        SpriteType::Wall => 0,
        SpriteType::Floor => 1,
        SpriteType::Exit => 2,
        SpriteType::DownControl => 34,
        SpriteType::UpControl => 32,
    };
    let row = index / NUM_SPRITES_X;
    let col = index % NUM_SPRITES_X;
    assert!(row < NUM_SPRITES_Y);
    Rect::new(
        col as f32 / NUM_SPRITES_X as f32,
        row as f32 / NUM_SPRITES_Y as f32,
        1.0 / NUM_SPRITES_X as f32,
        1.0 / NUM_SPRITES_Y as f32,
    )
}
