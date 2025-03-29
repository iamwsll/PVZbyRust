use std::path::PathBuf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};

mod game;
mod resources;
mod plant;
mod zombie;
mod sun;
mod grid;

const WINDOW_WIDTH: f32 = 1400.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("Resource");
        path
    } else {
        PathBuf::from("./Resource")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("pvz_rust", "wsll")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("植物大战僵尸 - Rust版"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let game_state = game::GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game_state)
}