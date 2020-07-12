mod game;
mod gfx;
mod level;

use ggez::conf::FullscreenType;
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, Drawable, Image};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{self, event::EventHandler, Context, GameResult};
use std::{env, path};

use game::{Control, ControlType, ItemType, Player, TileType};
use gfx::{get_sprite, SpriteType};
use level::Level;

const SCREEN_SIZE: (f32, f32) = (1000.0, 600.0);

struct MainState {
    sprite_sheet: Image,
    key_presses: Vec<KeyCode>,
    player: Player,
    level: Level,
    controls: [Option<Control>; 4],
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let level = Level::load(ctx, 0)?;
        Ok(MainState {
            sprite_sheet: Image::new(ctx, "/sprites.png")?,
            player: Player {
                pos: level.player_start,
                sprite: get_sprite(SpriteType::Player),
                pending_items: Vec::new(),
            },
            level,
            controls: [
                Some(Control {
                    energy: 15,
                    control_type: ControlType::Left,
                }),
                None,
                None,
                None,
            ],
            key_presses: Vec::new(),
        })
    }

    pub fn next_level(&mut self, ctx: &mut Context) {
        self.load_level(ctx, self.level.number + 1);
    }

    pub fn reload_level(&mut self, ctx: &mut Context) {
        self.load_level(ctx, self.level.number);
    }

    fn load_level(&mut self, ctx: &mut Context, level_number: usize) {
        self.level = Level::load(ctx, level_number).unwrap();
        self.player.pos = self.level.player_start;
        self.controls = self.level.controls_start.clone();
    }

    fn out_of_control(&self) -> bool {
        self.controls.iter().all(|c| c.is_none())
    }
}

fn add_control(controls: &mut [Option<Control>], item_type: ItemType) {
    let control_type = match item_type {
        ItemType::UpControl => ControlType::Up,
        ItemType::RightControl => ControlType::Right,
        ItemType::DownControl => ControlType::Down,
        ItemType::LeftControl => ControlType::Left,
    };
    let control = Control {
        energy: 5,
        control_type,
    };
    for control_holder in controls.iter_mut() {
        match control_holder {
            Some(_) => {}
            None => {
                *control_holder = Some(control);
                break;
            }
        }
    }
}

fn draw_number(number: usize, num_digits: usize, tile_pos: Point2<usize>, batch: &mut SpriteBatch) {
    assert!(number < 10_usize.pow(num_digits as u32));
    let mut remaining = number;
    let mut leading_zero = true;
    for position in (0..num_digits).rev() {
        let digit = remaining / 10_usize.pow(position as u32);
        remaining -= digit * 10_usize.pow(position as u32);
        match (digit, leading_zero) {
            (0, true) => {}
            (_, _) => {
                leading_zero = false;
                batch.add(DrawParam::default().src(gfx::get_digit_sprite(digit)).dest(
                    gfx::screen_pos(tile_pos + Vector2::new((num_digits - 1) - position, 0)),
                ));
            }
        }
    }
}

fn draw_control_status(controls: &[Option<Control>], pos: Point2<usize>, batch: &mut SpriteBatch) {
    let mut row = pos.y;
    let start_col = pos.x;
    let action_sprites = [SpriteType::H, SpriteType::J, SpriteType::K, SpriteType::L];
    for (index, sprite_type) in action_sprites.iter().enumerate() {
        batch.add(
            DrawParam::default()
                .src(gfx::get_sprite(*sprite_type))
                .dest(gfx::screen_pos(Point2::new(start_col, row))),
        );
        if let Some(Some(c)) = controls.get(index) {
            batch.add(
                DrawParam::default()
                    .src(gfx::get_sprite(match c.control_type {
                        ControlType::Up => SpriteType::UpControl,
                        ControlType::Right => SpriteType::RightControl,
                        ControlType::Down => SpriteType::DownControl,
                        ControlType::Left => SpriteType::LeftControl,
                    }))
                    .dest(gfx::screen_pos(Point2::new(start_col + 2, row))),
            );

            draw_number(c.energy as usize, 2, Point2::new(start_col + 4, row), batch);
        }
        row += 2;
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut restart = false;
        for keycode in self.key_presses.drain(..) {
            let control_index = match keycode {
                KeyCode::H => 0,
                KeyCode::J => 1,
                KeyCode::K => 2,
                KeyCode::L => 3,
                KeyCode::R => {
                    restart = true;
                    continue;
                }
                _ => continue,
            };
            if let Some(Some(control)) = self.controls.get_mut(control_index) {
                control.activate(&mut self.player, &mut self.level);
            }
        }
        self.controls.iter_mut().for_each(|c| {
            if let Some(control) = c {
                if !control.has_energy() {
                    *c = None
                }
            }
        });
        for item_type in self.player.pending_items.drain(..) {
            add_control(&mut self.controls, item_type);
        }
        if restart {
            self.reload_level(ctx);
        }
        if self
            .level
            .get(self.player.pos.x, self.player.pos.y)
            .unwrap()
            .tile_type
            == TileType::Exit
        {
            self.next_level(ctx);
        }
        if self.out_of_control() {
            panic!("You are out of controls! You lose!")
        }
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
                    .dest(gfx::screen_pos(Point2::new(x, y))),
            );
        }
        batch.add(
            DrawParam::default()
                .src(self.player.sprite)
                .dest(gfx::screen_pos(self.player.pos)),
        );
        draw_control_status(
            &self.controls,
            Point2::new(level::LEVEL_WIDTH + 1, 1),
            &mut batch,
        );
        batch
            .draw(
                ctx,
                DrawParam::default()
                    .scale([gfx::SPRITE_SCALE, gfx::SPRITE_SCALE])
                    .dest(Point2::new(
                        (SCREEN_SIZE.0
                            - (level::LEVEL_WIDTH + 7) as f32
                                * gfx::SPRITE_SCALE
                                * gfx::SPRITE_WIDTH as f32)
                            / 2.0,
                        (SCREEN_SIZE.1
                            - level::LEVEL_HEIGHT as f32
                                * gfx::SPRITE_SCALE
                                * gfx::SPRITE_WIDTH as f32)
                            / 2.0,
                    )),
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
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .fullscreen_type(FullscreenType::Desktop),
        );

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
