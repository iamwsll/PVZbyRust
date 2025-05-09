use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use std::{collections::HashMap, time::Instant};
use crate::core::resources::Resources;
use crate::ui::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH};
use crate::entities::sun::Sun;
use crate::entities::pea::Pea;

// 声明子模块
pub mod peashooter;
pub mod sunflower;
pub mod wallnut;
pub mod plant_trait;
pub mod plant_factory;

// 从工厂模块中重新导出植物类型枚举
pub use plant_factory::{PlantType, PlantFactory};

// 植物结构体，持有具体植物实现
pub struct Plant {
    pub grid_x: usize,
    pub grid_y: usize,
    pub health: i32,
    max_health: i32, // 记录最大生命值，用于计算损坏状态
    animation_frame: usize,
    animation_timer: u64,
    cooldown_timer: u64,
    pub is_dead: bool,
    plant_impl: Box<dyn plant_trait::PlantTrait>, // 实际植物实现
    plant_type: PlantType, // 记录植物类型
}

impl Plant {
    pub fn new(plant_type: PlantType, grid_x: usize, grid_y: usize) -> Self {
        // 使用工厂创建具体植物实现
        let mut plant_impl = PlantFactory::create_plant(plant_type);
        let health = plant_impl.get_initial_health();

        Plant {
            grid_x,
            grid_y,
            health,
            max_health: health, // 初始时最大生命值等于当前生命值
            animation_frame: 0,
            animation_timer: 0,
            cooldown_timer: 0,
            is_dead: false,
            plant_impl,
            plant_type,
        }
    }

    /// 用来更新植物状态
    /// @param dt: 距离上次更新的时间
    /// @param suns: 由于向日葵会产生阳光，所以需要传入阳光的引用
    /// @param peas: 由于豌豆射手会发射豌豆，所以需要传入豌豆的引用
    /// @return: None
    pub fn update(&mut self, dt: u64, suns: &mut Vec<Sun>, peas: &mut Vec<Pea>) {
        if self.is_dead {
            return; // 如果植物已经死亡，跳过更新
        }

        // 动画更新
        self.animation_timer += dt;
        if self.animation_timer > 100 { // 每100ms更新一次帧动画
            // 获取植物动画帧数
            let frame_count = self.plant_impl.get_frame_count();
            if frame_count > 0 {
                self.animation_frame = (self.animation_frame + 1) % frame_count;
            }
            self.animation_timer = 0;
        }

        // 冷却更新和动作执行
        let cooldown = self.plant_impl.get_cooldown();
        if cooldown > 0 {
            self.cooldown_timer += dt;
            if self.cooldown_timer >= cooldown {
                self.cooldown_timer = 0; // 重置计时器

                // 调用特定植物的 update_action 方法
                self.plant_impl.update_action(self.grid_x, self.grid_y, suns, peas);
            }
        }
        
        // 检查特殊效果
        self.plant_impl.special_effect(self.grid_x, self.grid_y);
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算植物在屏幕上的位置（加上少许偏移）
        let x = GRID_START_X + (self.grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 4.0;
        let y = GRID_START_Y + (self.grid_y as f32) * GRID_CELL_HEIGHT + GRID_CELL_HEIGHT / 4.0;

        // 获取当前植物状态对应的图像
        let image = self.plant_impl.get_current_frame_image(resources, self.animation_frame);

        // 绘制图像
        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([x, y])
                .scale([0.8, 0.8]),
        )
    }

    // 植物受伤方法
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;
        
        // 检查植物是否死亡
        if self.health <= 0 {
            self.is_dead = true;
            return true;  // 返回true表示植物已死亡
        }
        
        false  // 返回false表示植物仍然存活
    }
    
    // 获取植物的损坏状态（如坚果墙的不同损坏阶段）
    pub fn get_damage_state(&self) -> usize {
        self.plant_impl.get_damage_state(self.health, self.max_health)
    }
    
    // 获取植物类型
    pub fn get_plant_type(&self) -> PlantType {
        self.plant_type
    }
}
