//! # 向日葵模块 (`sunflower`)
//!
//! 实现了游戏中核心的资源产生单位——向日葵。
//! 向日葵会周期性地产生阳光，供玩家收集并用于购买其他植物。

use crate::ui::grid::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X, GRID_START_Y};
use crate::entities::sun::{Sun, SunType};
use crate::entities::pea::Pea;
use crate::plants::plant_trait::PlantTrait;
use crate::core::resources::Resources;
use ggez::graphics;
use crate::zombies::Zombie;

/// 向日葵植物的结构体。
///
/// 目前没有特有状态，其行为完全由 `PlantTrait` 定义。
pub struct Sunflower;

impl Sunflower {
    /// 创建一个新的 `Sunflower` 实例。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Sunflower` 实例。
    pub fn new() -> Self {
        Sunflower
    }
}

/// 向日葵的初始生命值。
const INITIAL_HEALTH: i32 = 300;
/// 向日葵产生阳光的冷却时间（毫秒）。
const COOLDOWN: u64 = 24000; // 产生阳光间隔为24秒 (原为5000)
/// 种植向日葵所需的阳光花费。
const COST: i32 = 50;

impl PlantTrait for Sunflower {
    /// 获取向日葵的初始生命值。
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }

    /// 获取向日葵产生阳光的冷却时间。
    fn get_cooldown(&self) -> u64 {
        COOLDOWN
    }

    /// 获取向日葵动画的总帧数。
    fn get_frame_count(&self) -> usize {
        18 // 向日葵动画有18帧
    }

    /// 更新向日葵的动作，主要是产生阳光。
    ///
    /// 当冷却完成后，此方法被调用。
    /// 它会在向日葵附近的一个随机位置创建一个新的阳光 (`SunType::SunflowerGeneration`)，
    /// 并将其添加到游戏世界的阳光列表中。
    ///
    /// # Arguments
    ///
    /// * `grid_x` - 向日葵所在的网格x坐标。
    /// * `grid_y` - 向日葵所在的网格y坐标。
    /// * `_suns` - 阳光列表的引用 (向日葵不产生阳光，故未使用)。
    /// * `_peas` - 豌豆列表的引用 (向日葵不发射豌豆，故未使用)。
    /// * `_zombies` - 僵尸列表的引用 (向日葵的动作不依赖僵尸状态，故未使用)。
    fn update_action(&mut self, grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>, _peas: &mut Vec<Pea>, _zombies: &Vec<Zombie>) {
        // 计算阳光生成的位置 (在向日葵上方一点)
        let sun_x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 2.0;
        let sun_y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT; 

        // 创建新的阳光
        suns.push(Sun::new(sun_x, sun_y, SunType::SunflowerGeneration));
    }

    /// 获取种植向日葵所需的阳光花费。
    fn get_cost(&self) -> i32 {
        COST
    }

    /// 获取向日葵在商店中显示的卡片图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用，用于获取图像。
    ///
    /// # Returns
    ///
    /// 返回向日葵卡片图像的引用。
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image {
        &resources.sunflower_card
    }
    
    /// 获取向日葵当前动画帧对应的图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用，用于获取动画帧图像序列。
    /// * `animation_frame` - 当前需要显示的动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回当前动画帧图像的引用。如果图像资源未加载，则返回卡片图像作为备用。
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a graphics::Image {
        let frame_count = resources.sunflower_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.sunflower_images[safe_index]
        } else {
            // 如果没有图像，返回卡片
            &resources.sunflower_card
        }
    }
}
