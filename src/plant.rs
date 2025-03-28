use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
}

impl PlantType {
    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::WallNut => 50,
        }
    }
}

pub struct Plant {
    plant_type: PlantType,
    grid_x: usize,
    grid_y: usize,
    health: i32,
    animation_frame: usize,
    animation_timer: u64,
    cooldown: u64,
    cooldown_timer: u64,
}

impl Plant {
    pub fn new(plant_type: PlantType, grid_x: usize, grid_y: usize) -> Self {
        let health = match plant_type {
            PlantType::Peashooter => 300,
            PlantType::Sunflower => 200,
            PlantType::WallNut => 1500,
        };

        Plant {
            plant_type,
            grid_x,
            grid_y,
            health,
            animation_frame: 0,
            animation_timer: 0,
            cooldown: 0,
            cooldown_timer: 0,
        }
    }

    pub fn update(&mut self, dt: u64) {
        // 动画更新
        self.animation_timer += dt;
        if self.animation_timer > 200 {
            self.animation_frame = (self.animation_frame + 1) % 2;
            self.animation_timer = 0;
        }

        // 冷却更新
        if self.cooldown > 0 {
            self.cooldown_timer += dt;
            if self.cooldown_timer >= self.cooldown {
                match self.plant_type {
                    PlantType::Sunflower => {
                        // 生成阳光
                    }
                    PlantType::Peashooter => {
                        // 发射豌豆
                    }
                    _ => {}
                }
                self.cooldown_timer = 0;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        let x = 80.0 + (self.grid_x as f32) * 80.0;
        let y = 80.0 + (self.grid_y as f32) * 80.0;

        let image = match self.plant_type {
            PlantType::Peashooter => &resources.peashooter_images[self.animation_frame],
            PlantType::Sunflower => &resources.sunflower_images[self.animation_frame],
            _ => &resources.peashooter_images[0], // 临时使用，应该为每种植物添加图片
        };

        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([x, y])
                .scale([0.8, 0.8]),
        )
    }
}
