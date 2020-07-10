use ggez::graphics::{self, DrawParam, Drawable, Image};
use ggez::{self, event::EventHandler, Context, GameResult};
use std::{env, path};

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

struct MainState {
    sprite_sheet: Image,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        MainState {
            sprite_sheet: Image::new(ctx, "/sprites.png").unwrap(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::BLACK);
        self.sprite_sheet.draw(ctx, DrawParam::default()).unwrap();
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
