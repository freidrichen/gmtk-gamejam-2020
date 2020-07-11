mod level;

use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{self, event::EventHandler, Context, GameResult};
use std::{env, path};

use level::Level;

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
const NUM_SPRITES_X: f32 = 8.0;
const NUM_SPRITES_Y: f32 = 8.0;

enum ControlType {
    Right,
    Left,
}

struct Control {
    energy: u32,
    control_type: ControlType,
}

impl Control {
    fn activate(&mut self, player: &mut Player) {
        assert!(self.energy > 0);
        self.energy -= 1;
        match self.control_type {
            ControlType::Right => player.pos += Vector2::new(1, 0),
            ControlType::Left => player.pos -= Vector2::new(1, 0),
        };
    }

    fn has_energy(&self) -> bool {
        self.energy > 0
    }
}

struct Player {
    pos: Point2<usize>,
    src_rect: Rect,
}

impl Player {
    fn update(&mut self, _ctx: &Context) -> GameResult<()> {
        Ok(())
    }
}

struct MainState {
    sprite_sheet: Image,
    player: Player,
    level: Level,
    controls: [Option<Control>; 4],
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            sprite_sheet: Image::new(ctx, "/sprites.png")?,
            player: Player {
                pos: [5, 5].into(),
                src_rect: [0.5, 0.5, 1.0 / NUM_SPRITES_X, 1.0 / NUM_SPRITES_Y].into(),
            },
            level: Level::load(ctx, "/level.txt")?,
            controls: [
                Some(Control { energy: 13, control_type: ControlType::Right }),
                Some(Control { energy: 10, control_type: ControlType::Left }),
                None,
                None,
            ],
        })
    }
}

fn screen_pos(tile_pos: Point2<usize>) -> Point2<f32> {
    Point2::new(tile_pos.x as f32 * 8.0, tile_pos.y as f32 * 8.0)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.update(ctx)?;
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        if keys.contains(&ggez::input::keyboard::KeyCode::L) {
            if let Some(Some(control)) = self.controls.get_mut(0) {
                control.activate(&mut self.player);
            }
        }
        if keys.contains(&ggez::input::keyboard::KeyCode::H) {
            if let Some(Some(control)) = self.controls.get_mut(1) {
                control.activate(&mut self.player);
            }
        }
        self.controls.iter_mut().for_each(|c| {
            if let Some(control) = c {
                if !control.has_energy() {
                    *c = None
                }
            }});
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mut batch = SpriteBatch::new(self.sprite_sheet.clone());
        for tile_y in 0..self.level.height{
            for tile_x in 0..self.level.width {
                let screen_x = tile_x as f32 * 8.0;
                let screen_y = tile_y as f32 * 8.0;
                batch.add(
                    DrawParam::default()
                        .src(self.level.get(tile_x, tile_y).unwrap().src_rect)
                        .dest(Point2::new(screen_x, screen_y))
                );
            }
        }
        batch.add(
            DrawParam::default()
                .src(self.player.src_rect)
                .dest(screen_pos(self.player.pos)),
        );
        batch
            .draw(ctx, DrawParam::default().scale([2.0, 2.0]))
            .unwrap();
        graphics::present(ctx).unwrap();
        Ok(())
    }
}

fn main() {
    let mut cb = ggez::ContextBuilder::new("NoCtrl", "Weirdo")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("NoCtrl")
                .vsync(true),
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));

    // Setup 'resources' dir in cargo project dir.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, mut event_loop) = cb.build().unwrap();

    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let mut main_state = MainState::new(&mut ctx).unwrap();

    match ggez::event::run(&mut ctx, &mut event_loop, &mut main_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
