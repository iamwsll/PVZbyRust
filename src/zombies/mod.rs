use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Rect};
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
    pub row: usize,
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
            x: 950.0, // 从屏幕更右侧开始，确保完全在屏幕外生成
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
        if self.animation_timer > 200 { // 动画切换速度，每200ms绘制动画的一帧。
            // TODO: 根据僵尸类型和状态（行走/攻击）选择不同的动画帧范围
            let frame_count = 22; // 22帧
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
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0; // 示例 Y 坐标

        // 根据僵尸类型和状态选择图像 (未来可以更复杂)
        let image = match self.zombie_type {
            ZombieType::Normal => {
                // TODO: 根据 attacking 状态选择行走或攻击动画
                // 目前只使用行走动画
                let frame_count = resources.zombies_walk1_images.len();
                if frame_count > 0 {
                    &resources.zombies_walk1_images[self.animation_frame % frame_count]
                } else {
                    println!("No images available for Normal Zombie");
                    &resources.zombies_walk1_images[0] // 返回默认图像或处理错误
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

    // 获取僵尸的碰撞矩形
    pub fn get_rect(&self) -> Rect {
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0;
        
        // 僵尸的碰撞区域应该比显示的图像小一些，以使游戏更加公平 .TODO: 根据实际图像大小调整
        let width = 60.0;
        let height = 100.0;
        
        Rect::new(self.x + 40.0, y + 20.0, width, height)
    }
    
    // 僵尸受到伤害的方法
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;
        println!("僵尸受到{}点伤害，剩余生命值: {}", damage, self.health);
        
        // 返回僵尸是否死亡
        self.health <= 0
    }
    
}
