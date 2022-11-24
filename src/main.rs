use ggez;
use ggez::{ graphics, Context, GameResult, timer };
use ggez::event::{ self, EventHandler, MouseButton };
use ggez::nalgebra as na;

use std::env;
use std::path;

struct CircleObject {
    pos_x: f32,
    pos_y: f32,
}

impl CircleObject {
    // pub fn new(pos_x: f32, pos_y: f32) -> Self {
    //   Bulet {
    //     pos_x,
    //     pos_y
    //   }
    // }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, self.pos_y),
            50.0,
            0.2,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}

struct MainState {
    circles: Vec<CircleObject>,
    text: graphics::Text,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/PressStart2P.ttf")?;
        let text = graphics::Text::new(("underpost.net \n on Rust", font, 30.0));
        let main_state = MainState {
            circles: Vec::new(),
            text,
        };
        Ok(main_state)
    }

    fn clean_bulets(&mut self) {
        self.circles.retain(|b| b.pos_y < 500.0);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const FPS: u32 = 24;

        while timer::check_update_time(_ctx, FPS) {
            let seconds = 1.0 / (FPS as f32);

            for b in &mut self.circles {
                b.pos_y += 100.0 * seconds;
            }

            self.clean_bulets();
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, [0.1, 0.2, 0.3, 1.0].into());

        for b in &self.circles {
            b.draw(_ctx)?;
        }

        let dest_point = na::Point2::new(300.0, 300.0);
        graphics::draw(_ctx, &self.text, (dest_point,))?;

        graphics::present(_ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        match _button {
            MouseButton::Left => {
                self.circles.push(CircleObject {
                    pos_x: _x,
                    pos_y: _y,
                });
            }
            _ => {
                println!("Other button is clicked");
            }
        }
    }
}

fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb = ggez::ContextBuilder::new("mouse_event", "miqdude").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}