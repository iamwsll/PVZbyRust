use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;

// 声明子模块
pub mod normal;
// pub mod conehead; // 未来可以添加
// pub mod buckethead; // 未来可以添加

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZombieType {
    Normal,
    // Conehead,
    // Buckethead,
}

pub struct Zombie {
    zombie_type: ZombieType,
    row: usize,
    pub x: f32, // Make x public for game logic access
    health: i32,
    speed: f32,
    animation_frame: usize,
    animation_timer: u64,
    attacking: bool, // 僵尸是否在攻击 (未来用于与植物交互)
}

impl Zombie {
    pub fn new(zombie_type: ZombieType, row: usize) -> Self {
        let (health, speed) = match zombie_type {
            ZombieType::Normal => (normal::INITIAL_HEALTH, normal::SPEED),
            // ZombieType::Conehead => (conehead::INITIAL_HEALTH, conehead::SPEED),
            // ZombieType::Buckethead => (buckethead::INITIAL_HEALTH, buckethead::SPEED),
        };

        Zombie {
            zombie_type,
            row,
            x: 800.0, // 从屏幕更右侧开始，确保完全在屏幕外生成
            health,
            speed,
            animation_frame: 0,
            animation_timer: 0,
            attacking: false,
        }
    }

    pub fn update(&mut self, dt: u64) {
        // 动画更新 (通用逻辑)
        self.animation_timer += dt;
        if self.animation_timer > 200 { // 动画切换速度
            // TODO: 根据僵尸类型和状态（行走/攻击）选择不同的动画帧范围
            let frame_count = 2; // 假设行走动画有2帧
            self.animation_frame = (self.animation_frame + 1) % frame_count;
            self.animation_timer = 0;
        }

        // 移动逻辑 (通用逻辑)
        if !self.attacking {
            self.x -= self.speed * dt as f32;
        }

        // TODO: 调用特定僵尸的更新逻辑 (如果需要)
        // match self.zombie_type {
        //     ZombieType::Normal => normal::update(),
        //     ...
        // }

        // TODO: 添加攻击逻辑检查 (当僵尸遇到植物时，设置 attacking = true)
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算僵尸在屏幕上的 Y 坐标 (通用逻辑)
        // 需要根据实际网格和僵尸图像调整 Y 坐标和偏移量
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT + 15.0; // 示例 Y 坐标

        // 根据僵尸类型和状态选择图像 (未来可以更复杂)
        let image = match self.zombie_type {
            ZombieType::Normal => {
                // TODO: 根据 attacking 状态选择行走或攻击动画
                // 目前只使用行走动画
                let images = &resources.zombie_walk_images; // 假设有一个行走动画序列
                if !images.is_empty() {
                    &images[self.animation_frame % images.len()]
                } else {
                    panic!("Normal zombie walk images not loaded!");
                }
            }
            // Handle other zombie types...
        };

        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([self.x, y])
                .scale([0.8, 0.8]), // 僵尸图像缩放比例
        )
    }

    // 添加 getter 方法以访问私有字段
    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_rect(&self, resources: &Resources) -> graphics::Rect {
        // 获取当前僵尸图像的尺寸用于碰撞检测
        let image = match self.zombie_type {
             ZombieType::Normal => {
                let images = &resources.zombie_walk_images;
                if !images.is_empty() {
                    &images[self.animation_frame % images.len()]
                } else {
                     // 返回一个默认矩形或处理错误
                     return graphics::Rect::new(self.x, 0.0, 0.0, 0.0);
                }
            }
            // ... 其他僵尸类型
        };
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT + 15.0;
        let scale = [0.8, 0.8]; // 与 draw 中使用的缩放一致
        let width = image.width() as f32 * scale[0];
        let height = image.height() as f32 * scale[1];
        // 创建一个稍微调整过的矩形用于碰撞检测，可以根据需要微调
        graphics::Rect::new(self.x + width * 0.1, y, width * 0.8, height)
    }


    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
