//! # 豌豆射手模块 (`peashooter`)
//!
//! 实现了游戏中基础的远程攻击单位——豌豆射手。
//! 豌豆射手会周期性地向其所在行的前方发射豌豆来攻击僵尸。

use crate::ui::grid::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X, GRID_START_Y};
use crate::entities::sun::Sun;
use crate::entities::pea::{Pea, PeaType};
use crate::plants::plant_trait::PlantTrait;
use crate::core::resources::Resources;
use ggez::graphics;

/// 豌豆射手植物的结构体。
///
/// 包含其特有的状态，例如 `shoot_timer` 用于控制发射豌豆的冷却。
pub struct Peashooter {
    /// 射击计时器，用于追踪距离下次发射豌豆还需多少时间。
    shoot_timer: u64,
}

impl Peashooter {
    /// 创建一个新的 `Peashooter` 实例。
    ///
    /// 初始化时，射击计时器 `shoot_timer` 设置为0。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Peashooter` 实例。
    pub fn new() -> Self {
        Peashooter {
            shoot_timer: 0,
        }
    }
}

/// 豌豆射手的初始生命值。
const INITIAL_HEALTH: i32 = 300;
/// 豌豆射手发射豌豆的冷却时间（毫秒）。
const COOLDOWN: u64 = 1400; // 发射间隔为1.4秒
/// 种植豌豆射手所需的阳光花费。
const COST: i32 = 100;

impl PlantTrait for Peashooter {
    /// 获取豌豆射手的初始生命值。
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    /// 获取豌豆射手发射豌豆的冷却时间。
    fn get_cooldown(&self) -> u64 {
        COOLDOWN
    }
    
    /// 获取豌豆射手动画的总帧数。
    fn get_frame_count(&self) -> usize {
        13 // 豌豆射手动画有13帧
    }
    
    /// 更新豌豆射手的动作，主要是发射豌豆。
    ///
    /// 当冷却完成后，此方法被调用。
    /// 它会在豌豆射手前方创建一个新的普通豌豆 (`PeaType::Normal`)，
    /// 并将其添加到游戏世界的豌豆列表中。
    /// 然后重置射击计时器。
    ///
    /// # Arguments
    ///
    /// * `grid_x` - 豌豆射手所在的网格x坐标。
    /// * `grid_y` - 豌豆射手所在的网格y坐标。
    /// * `_suns` - 阳光列表的引用 (豌豆射手不产生阳光，故未使用)。
    /// * `peas` - 一个可变向量的引用，用于添加新发射的豌豆。
    fn update_action(&mut self, grid_x: usize, grid_y: usize, _suns: &mut Vec<Sun>, peas: &mut Vec<Pea>) {
        // 计算豌豆射手的位置，用于确定豌豆的发射位置
        let x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH * 0.8;
        let y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT + GRID_CELL_HEIGHT * 0.3;
        
        // 创建一个新豌豆
        let new_pea = Pea::new(x, y, grid_y, PeaType::Normal);
        
        // 添加到豌豆列表中
        peas.push(new_pea);
        
        // 重置发射计时器
        self.shoot_timer = 0;
    }
    
    /// 获取种植豌豆射手所需的阳光花费。
    fn get_cost(&self) -> i32 {
        COST
    }
    
    /// 获取豌豆射手在商店中显示的卡片图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用，用于获取图像。
    ///
    /// # Returns
    ///
    /// 返回豌豆射手卡片图像的引用。
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image {
        &resources.peashooter_card
    }
    
    /// 获取豌豆射手当前动画帧对应的图像。
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
        let frame_count = resources.peashooter_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.peashooter_images[safe_index]
        } else {
            // 如果没有图像，返回卡片
            &resources.peashooter_card
        }
    }
}