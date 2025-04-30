use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;
use crate::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT,GRID_CELL_WIDTH};
use crate::sun::Sun; // 保持 Sun 导入

// Declare submodules and import their update functions
pub mod peashooter;
pub mod sunflower;
pub mod wallnut;

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
            PlantType::Peashooter => peashooter::INITIAL_HEALTH, // Use specific health
            PlantType::Sunflower => sunflower::INITIAL_HEALTH, // Use specific health
            PlantType::WallNut => wallnut::INITIAL_HEALTH,   // Use specific health
        };

        // Get specific cooldown from submodules if needed
        let cooldown = match plant_type {
             PlantType::Peashooter => peashooter::COOLDOWN,
             PlantType::Sunflower => sunflower::COOLDOWN,
             _ => 0, // WallNut might not have a cooldown for actions
        };


        Plant {
            plant_type,
            grid_x,
            grid_y,
            health,
            animation_frame: 0,
            animation_timer: 0,
            cooldown, // Use specific cooldown
            cooldown_timer: 0,
        }
    }

    pub fn update(&mut self, dt: u64, suns: &mut Vec<Sun>) {
        // 动画更新 (通用逻辑)
        self.animation_timer += dt;
        if self.animation_timer > 200 { // 动画速度可以考虑植物特定化
            self.animation_frame = (self.animation_frame + 1) % 2; // 假设目前只有2帧
            self.animation_timer = 0;
        }

        // 冷却更新和动作执行 (委托给子模块)
        if self.cooldown > 0 {
            self.cooldown_timer += dt;
            if self.cooldown_timer >= self.cooldown {
                self.cooldown_timer = 0; // 重置计时器

                // 调用特定植物的 update 函数，传入 suns
                match self.plant_type {
                    PlantType::Sunflower => {
                        // 向日葵直接修改 suns
                        sunflower::update(self.grid_x, self.grid_y, suns);
                    },
                    PlantType::Peashooter => peashooter::update(self.grid_x, self.grid_y),
                    PlantType::WallNut => wallnut::update(self.grid_x, self.grid_y),
                };
            }
        }

        // TODO: 添加特定更新逻辑调用，例如坚果墙的损坏状态检查 (可以在 wallnut::update 中实现)
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算植物在屏幕上的位置,最后一项是偏移量 (Common logic)
        let x = GRID_START_X + (self.grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 4.0;
        let y = GRID_START_Y + (self.grid_y as f32) * GRID_CELL_HEIGHT+ GRID_CELL_HEIGHT / 4.0;

        // Get the correct image based on type and state (Potentially delegate?)
        // For now, keep the simple frame logic here
        let image = match self.plant_type {
             // TODO: Potentially delegate image selection to submodules for complex animations (e.g., WallNut damage)
            PlantType::Peashooter => {
                let frame_count = resources.peashooter_images.len();
                if frame_count > 0 {
                    &resources.peashooter_images[self.animation_frame % frame_count]
                } else {
                    // 如果没有图片，返回一个错误或默认图片
                    // 这里暂时 panic，因为资源未加载是严重错误
                    panic!("Peashooter images not loaded!");
                }
            },
            PlantType::Sunflower => {
                let frame_count = resources.sunflower_images.len();
                if frame_count > 0 {
                    &resources.sunflower_images[self.animation_frame % frame_count] // 使用模运算防止越界
                } else {
                    panic!("Sunflower images not loaded!");
                }
            },
            PlantType::WallNut => {
                let frame_count = resources.wallnut_images.len();
                if frame_count > 0 {
                    &resources.wallnut_images[self.animation_frame % frame_count] // 使用模运算防止越界
                } else {
                    panic!("WallNut images not loaded!");
                }
            },
        };

        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([x, y])
                .scale([0.8, 0.8]), // Consider making scale plant-specific?
        )
    }

    // Add getter methods if needed, e.g., to access position or type
    pub fn plant_type(&self) -> PlantType {
        self.plant_type
    }

    pub fn grid_pos(&self) -> (usize, usize) {
        (self.grid_x, self.grid_y)
    }

    // Add method for taking damage
    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        // TODO: Add logic for WallNut damage states based on health (could be in wallnut::update or here)
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
