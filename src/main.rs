use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{self, event::EventHandler, Context, GameResult};
use std::{env, path};

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
const NUM_SPRITES_X: f32 = 8.0;
const NUM_SPRITES_Y: f32 = 8.0;

trait Control {
    fn activate(&mut self, player:&mut Player) -> GameResult<()>;
}

struct RightControl {}

impl Control for RightControl {
    fn activate(&mut self, player: &mut Player) -> GameResult<()> {
        player.pos += Vector2::new(10.0, 0.0);
        Ok(())
    }
}

struct Player {
    pos: Point2<f32>,
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
    controls: Vec<Box<dyn Control>>
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        MainState {
            sprite_sheet: Image::new(ctx, "/sprites.png").unwrap(),
            player: Player {
                pos: [50.0, 50.0].into(),
                src_rect: [0.0, 0.0, 1.0 / NUM_SPRITES_X, 1.0 / NUM_SPRITES_Y].into(),
            },
            controls: vec![Box::new(RightControl{})],
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.update(ctx)?;
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        if keys.contains(&ggez::input::keyboard::KeyCode::L) {
            self.controls[0].activate(&mut self.player)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mut batch = SpriteBatch::new(self.sprite_sheet.clone());
        batch.add(DrawParam::default().src(self.player.src_rect).dest(self.player.pos));
        batch.draw(ctx, DrawParam::default().scale([2.0, 2.0])).unwrap();
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

    let mut main_state = MainState::new(&mut ctx);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut main_state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
