mod gfx;
mod level;

use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, Drawable, Image};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{self, event::EventHandler, Context, GameResult};
use std::{env, path};

use level::Level;

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
use gfx::{get_sprite, Sprite, SpriteType};

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
    sprite: Sprite,
}

impl Player {
    fn update(&mut self, _ctx: &Context) -> GameResult<()> {
        Ok(())
    }
}

struct MainState {
    sprite_sheet: Image,
    key_presses: Vec<KeyCode>,
    player: Player,
    level: Level,
    controls: [Option<Control>; 4],
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let level = Level::load(ctx, "/level.txt")?;
        Ok(MainState {
            sprite_sheet: Image::new(ctx, "/sprites.png")?,
            player: Player {
                pos: level.player_start,
                sprite: get_sprite(SpriteType::Player),
            },
            level,
            controls: [
                Some(Control {
                    energy: 10,
                    control_type: ControlType::Left,
                }),
                None,
                None,
                Some(Control {
                    energy: 13,
                    control_type: ControlType::Right,
                }),
            ],
            key_presses: Vec::new(),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.update(ctx)?;
        for keycode in self.key_presses.drain(..) {
            let control_index = match keycode {
                KeyCode::H => 0,
                KeyCode::J => 1,
                KeyCode::K => 2,
                KeyCode::L => 3,
                _ => continue,
            };
            if let Some(Some(control)) = self.controls.get_mut(control_index) {
                control.activate(&mut self.player);
            }
        }
        self.controls.iter_mut().for_each(|c| {
            if let Some(control) = c {
                if !control.has_energy() {
                    *c = None
                }
            }
        });
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mut batch = SpriteBatch::new(self.sprite_sheet.clone());
        for tile_y in 0..self.level.height {
            for tile_x in 0..self.level.width {
                batch.add(
                    DrawParam::default()
                        .src(self.level.get(tile_x, tile_y).unwrap().sprite)
                        .dest(gfx::screen_pos([tile_x, tile_y].into())),
                );
            }
        }
        for (&(x, y), item) in &self.level.items {
            batch.add(
                DrawParam::default()
                .src(item.sprite)
                .dest(gfx::screen_pos(Point2::new(x,y))));
        }
        batch.add(
            DrawParam::default()
                .src(self.player.sprite)
                .dest(gfx::screen_pos(self.player.pos)),
        );
        batch
            .draw(
                ctx,
                DrawParam::default().scale([gfx::SPRITE_SCALE, gfx::SPRITE_SCALE]),
            )
            .unwrap();
        graphics::present(ctx).unwrap();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.key_presses.push(keycode);
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
