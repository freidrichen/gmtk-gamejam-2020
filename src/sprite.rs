use ggez::graphics::Rect;

const NUM_SPRITES_X: usize = 8;
const NUM_SPRITES_Y: usize = 8;

pub type Sprite = Rect;

pub enum SpriteType {
    Player,
    Wall,
    Floor,
}

pub fn get_sprite(sprite_type: SpriteType) -> Rect {
    let index = match sprite_type {
        SpriteType::Player => 2,
        SpriteType::Wall => 1,
        SpriteType::Floor => 0,
    };
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
