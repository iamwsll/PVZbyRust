use std::path::PathBuf;
use ggez::{ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};

// 重组后的模块结构
mod core;
mod entities;
mod plants;
mod zombies;
mod mechanics;
mod ui;

// 设计时的窗口尺寸，作为缩放参考基准
pub const DESIGN_WIDTH: f32 = 1400.0;
pub const DESIGN_HEIGHT: f32 = 600.0;


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
        .window_mode(WindowMode::default()
            .dimensions(DESIGN_WIDTH, DESIGN_HEIGHT)
            // .resizable(true)
            )
        .build()?;

    let game_state = core::game::GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game_state)
}