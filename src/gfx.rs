use ggez::graphics::Rect;
use ggez::nalgebra::Point2;

const NUM_SPRITES_X: usize = 8;
const NUM_SPRITES_Y: usize = 8;
const SPRITE_WIDTH: usize = 8;
const SPRITE_HEIGHT: usize = 8;
pub const SPRITE_SCALE: f32 = 4.0;

pub type Sprite = Rect;

#[derive(Clone, Copy)]
pub enum SpriteType {
    Player,
    Wall,
    Floor,
    Exit,
    UpControl,
    RightControl,
    DownControl,
    LeftControl,
    H,
    J,
    K,
    L,
    Digit0,
    // Rest of digits are consecutive and are computed when needed.
}

pub fn screen_pos(tile_pos: Point2<usize>) -> Point2<f32> {
    Point2::new(
        (tile_pos.x * SPRITE_WIDTH) as f32,
        (tile_pos.y * SPRITE_HEIGHT) as f32,
    )
}

fn sprite_index(sprite_type: SpriteType) -> usize {
    match sprite_type {
        SpriteType::Player => 8,
        SpriteType::Wall => 0,
        SpriteType::Floor => 1,
        SpriteType::Exit => 2,
        SpriteType::UpControl => 32,
        SpriteType::RightControl => 33,
        SpriteType::DownControl => 34,
        SpriteType::LeftControl => 35,
        SpriteType::H => 16,
        SpriteType::J => 17,
        SpriteType::K => 18,
        SpriteType::L => 19,
        SpriteType::Digit0 => 54,
    }
}

fn sprite(sprite_index: usize) -> Rect {
    let row = sprite_index / NUM_SPRITES_X;
    let col = sprite_index % NUM_SPRITES_X;
    assert!(row < NUM_SPRITES_Y);
    Rect::new(
        col as f32 / NUM_SPRITES_X as f32,
        row as f32 / NUM_SPRITES_Y as f32,
        1.0 / NUM_SPRITES_X as f32,
        1.0 / NUM_SPRITES_Y as f32,
    )
}

pub fn get_sprite(sprite_type: SpriteType) -> Rect {
    let index = sprite_index(sprite_type);
    sprite(index)
}

pub fn get_digit_sprite(digit: usize) -> Rect {
    assert!(digit < 10);
    let index = sprite_index(SpriteType::Digit0) + digit;
    sprite(index)
}
