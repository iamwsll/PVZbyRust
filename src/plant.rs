use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;
use crate::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT,GRID_CELL_WIDTH};
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
    // 动画帧 目前没用?
    animation_frame: usize,
    animation_timer: u64,
    //这个冷却时间是指如豌豆射手的发射时间间隔
    cooldown: u64,
    cooldown_timer: u64,
}

impl Plant {
    pub fn new(plant_type: PlantType, grid_x: usize, grid_y: usize) -> Self {
        let health = match plant_type {
            PlantType::Peashooter => 300,
            PlantType::Sunflower => 300,
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
        // 计算植物在屏幕上的位置,最后一项是偏移量
        let x = GRID_START_X + (self.grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 4.0;
        let y = GRID_START_Y + (self.grid_y as f32) * GRID_CELL_HEIGHT+ GRID_CELL_HEIGHT / 4.0;

        let image = match self.plant_type {
            PlantType::Peashooter => &resources.peashooter_images[self.animation_frame],
            PlantType::Sunflower => &resources.sunflower_images[self.animation_frame],
            PlantType::WallNut => &resources.wallnut_images[self.animation_frame],
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
