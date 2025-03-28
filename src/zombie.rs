use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;

#[derive(Debug, Clone, Copy)]
pub enum ZombieType {
    Normal,
    Conehead,
    Buckethead,
}

pub struct Zombie {
    zombie_type: ZombieType,
    row: usize,
    x: f32,
    health: i32,
    speed: f32,
    animation_frame: usize,
    animation_timer: u64,
    attacking: bool,
}

impl Zombie {
    pub fn new(zombie_type: ZombieType, row: usize) -> Self {
        let (health, speed) = match zombie_type {
            ZombieType::Normal => (200, 0.2),
            ZombieType::Conehead => (560, 0.2),
            ZombieType::Buckethead => (1300, 0.15),
        };

        Zombie {
            zombie_type,
            row,
            x: 800.0,  // 从屏幕右侧开始
            health,
            speed,
            animation_frame: 0,
            animation_timer: 0,
            attacking: false,
        }
    }

    pub fn update(&mut self, dt: u64) {
        // 动画更新
        self.animation_timer += dt;
        if self.animation_timer > 200 {
            self.animation_frame = (self.animation_frame + 1) % 2;
            self.animation_timer = 0;
        }

        // 移动逻辑
        if !self.attacking {
            self.x -= self.speed * dt as f32;
        }
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        let y = 60.0 + (self.row as f32) * 80.0;

        graphics::draw(
            ctx,
            &resources.zombie_images[self.animation_frame],
            DrawParam::default()
                .dest([self.x, y])
                .scale([0.8, 0.8]),
        )
    }
}
