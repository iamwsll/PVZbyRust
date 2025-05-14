//! # 植物模块 (`plants`)
//!
//! 本模块定义了游戏中所有类型的植物，包括它们的行为、属性、以及与游戏世界的交互方式。
//!
//! ## 主要组成部分：
//! - **具体植物实现 (`peashooter`, `sunflower`, `wallnut`)**: 每个子模块实现了特定植物的逻辑，例如豌豆射手的攻击、向日葵产生阳光、坚果墙的防御等。
//! - **植物特征 (`plant_trait`)**: 定义了一个 `PlantTrait`，所有具体植物都需要实现此特征，以提供统一的接口供游戏逻辑调用（如获取生命值、冷却时间、执行动作等）。
//! - **植物工厂 (`plant_factory`)**: 提供了一个工厂模式 (`PlantFactory`)，用于根据植物类型 (`PlantType` 枚举) 创建具体的植物实例。
//!
//! ## 通用植物结构 (`Plant`):
//! 此模块还定义了一个通用的 `Plant` 结构体，它封装了所有植物共有的属性（如网格位置、生命值、动画状态等），
//! 并持有一个实现了 `PlantTrait` 的具体植物实例 (`Box<dyn PlantTrait>`)，通过这种方式实现多态。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::core::resources::Resources;
use crate::ui::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH};
use crate::entities::sun::Sun;
use crate::entities::pea::Pea;
use crate::zombies::Zombie; 

// 声明子模块
/// 豌豆射手植物的实现。
pub mod peashooter;
/// 向日葵植物的实现。
pub mod sunflower;
/// 坚果墙植物的实现。
pub mod wallnut;
/// 定义了所有植物应共享的行为特征 (`PlantTrait`)。
pub mod plant_trait;
/// 植物工厂，用于创建不同类型的植物实例。
pub mod plant_factory;

// 从工厂模块中重新导出植物类型枚举和工厂本身
pub use plant_factory::{PlantType, PlantFactory};

/// 通用植物结构体，代表游戏中的一个已种植的植物。
///
/// 它包含了植物在网格中的位置、当前生命值、最大生命值、动画相关状态、
/// 冷却计时器、是否死亡等通用属性。核心在于 `plant_impl` 字段，
/// 它是一个动态分派的 `PlantTrait` 对象，持有了具体植物类型的实现逻辑。
pub struct Plant {
    /// 植物所在的网格x坐标（列索引）。
    pub grid_x: usize,
    /// 植物所在的网格y坐标（行索引）。
    pub grid_y: usize,
    /// 植物当前的生命值。
    pub health: i32,
    /// 植物的最大生命值，用于计算损坏状态或UI显示。
    max_health: i32,
    /// 当前显示的动画帧索引。
    animation_frame: usize,
    /// 动画帧切换的计时器。
    animation_timer: u64,
    /// 植物特殊动作（如攻击、产生阳光）的冷却计时器。
    cooldown_timer: u64,
    /// 标记植物是否已经死亡。
    pub is_dead: bool,
    /// 持有具体植物行为逻辑的 `PlantTrait` 对象。
    plant_impl: Box<dyn plant_trait::PlantTrait>,
    /// 植物的类型，用于区分不同种类的植物。
    plant_type: PlantType,
}

impl Plant {
    /// 创建一个新的 `Plant` 实例。
    ///
    /// 使用 `PlantFactory` 根据指定的 `plant_type` 创建具体的植物实现 (`plant_impl`)，
    /// 并初始化其生命值等通用属性。
    ///
    /// # Arguments
    ///
    /// * `plant_type` - 要创建的植物的类型 (`PlantType`)。
    /// * `grid_x` - 植物放置的网格x坐标。
    /// * `grid_y` - 植物放置的网格y坐标。
    ///
    /// # Returns
    ///
    /// 返回一个新创建的 `Plant` 实例。
    pub fn new(plant_type: PlantType, grid_x: usize, grid_y: usize) -> Self {
        // 使用工厂创建具体植物实现
        let plant_impl = PlantFactory::create_plant(plant_type);
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

    /// 更新植物的状态，包括动画、冷却和执行特定动作。
    ///
    /// 如果植物未死亡：
    /// 1. 更新动画计时器和动画帧。
    /// 2. 更新冷却计时器；如果冷却完成，则调用具体植物实现的 `update_action` 方法，
    ///    并重置冷却计时器。`update_action` 可能会产生新的阳光或豌豆。
    /// 3. 调用具体植物实现的 `special_effect` 方法（如果存在）。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒）。
    /// * `suns` - 一个可变向量的引用，用于收集由向日葵等植物产生的阳光。
    /// * `peas` - 一个可变向量的引用，用于收集由豌豆射手等植物发射的豌豆。
    /// * `zombies` - 一个不可变的引用，代表当前场上所有僵尸的状态。
    pub fn update(&mut self, dt: u64, suns: &mut Vec<Sun>, peas: &mut Vec<Pea>, zombies: &Vec<Zombie>) {
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
                self.plant_impl.update_action(self.grid_x, self.grid_y, suns, peas, zombies);
            }
        }
        
        // 检查特殊效果
        self.plant_impl.special_effect(self.grid_x, self.grid_y);
    }

    /// 绘制植物到屏幕上。
    ///
    /// 计算植物在屏幕上的精确绘制位置，并调用具体植物实现的 `get_current_frame_image`
    /// 来获取当前应显示的动画帧图像进行绘制。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `resources` - 游戏资源，用于获取植物图像。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
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

    /// 使植物受到指定量的伤害。
    ///
    /// 减少植物的 `health` 值。如果生命值降至0或以下，将植物标记为 `is_dead`。
    ///
    /// # Arguments
    ///
    /// * `damage` - 对植物造成的伤害值。
    ///
    /// # Returns
    ///
    /// 返回一个布尔值，如果植物因此次伤害而死亡，则为 `true`，否则为 `false`。
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;
        
        // 检查植物是否死亡
        if self.health <= 0 {
            self.is_dead = true;
            return true;  // 返回true表示植物已死亡
        }
        
        false  // 返回false表示植物仍然存活
    }
    
    /// 获取植物的损坏状态。
    ///
    /// 此方法通常用于像坚果墙这类有多个损坏阶段视觉表现的植物。
    /// 它基于当前生命值与最大生命值的比例来决定返回哪个状态索引。
    /// 具体逻辑由植物的 `PlantTrait` 实现中的 `get_damage_state_count` 定义。
    ///
    /// # Returns
    ///
    /// 返回一个 `usize` 值，代表植物当前的损坏状态等级。
    pub fn get_damage_state(&self) -> usize {
        self.plant_impl.get_damage_state()
    }
    
    /// 获取植物的类型。
    ///
    /// # Returns
    ///
    /// 返回当前植物的 `PlantType` 枚举成员。
    pub fn get_plant_type(&self) -> PlantType {
        self.plant_type
    }
}
