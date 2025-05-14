//! # 坚果墙模块 (`wallnut`)
//!
//! 实现了游戏中的高生命值防御单位——坚果墙。
//! 坚果墙本身不具备攻击能力，其主要作用是阻挡僵尸前进，为后方植物提供保护。
//! 它具有多个损坏阶段的视觉表现。

use crate::core::resources::Resources;
use crate::entities::pea::Pea;
use crate::entities::sun::Sun;
use crate::plants::plant_trait::PlantTrait;
use ggez::graphics;
use crate::zombies::Zombie;

/// 坚果墙植物的结构体。
///
/// 目前没有特有状态，其行为和属性由 `PlantTrait` 和相关常量定义。
pub struct WallNut;

impl WallNut {
    /// 创建一个新的 `WallNut` 实例。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `WallNut` 实例。
    pub fn new() -> Self {
        WallNut
    }
}

/// 坚果墙的初始生命值。
const INITIAL_HEALTH: i32 = 4000;
/// 坚果墙的冷却时间（对于坚果墙来说，此值通常不用于主动技能，可能表示再次种植的冷却）。
const COOLDOWN: u64 = 30000; // 坚果墙冷却时间较长
/// 种植坚果墙所需的阳光花费。
const COST: i32 = 50;
/// 坚果墙动画的总帧数（通常坚果墙的损坏阶段不通过单一连续动画帧表示，而是通过切换不同状态的图像集）。
const FRAME_COUNT: usize = 16; // 假设一个损坏阶段有16帧动画，或者这是主要形态的帧数
/// 坚果墙具有的损坏状态数量（例如：完好、轻微损坏、严重损坏）。
const DAMAGE_STATE_COUNT: usize = 3; // 坚果墙有3个损坏阶段

impl PlantTrait for WallNut {
    /// 获取坚果墙的初始生命值。
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }

    /// 获取坚果墙的冷却时间。
    fn get_cooldown(&self) -> u64 {
        COOLDOWN
    }

    /// 获取坚果墙（某一状态下）的动画帧数。
    fn get_frame_count(&self) -> usize {
        FRAME_COUNT
    }

    /// 坚果墙的更新动作。
    ///
    /// 坚果墙没有主动的 `update_action`（如攻击或产生阳光）。
    /// 此方法为空实现。
    ///
    /// # Arguments
    ///
    /// * `_grid_x` - 植物所在的网格x坐标 (未使用)。
    /// * `_grid_y` - 植物所在的网格y坐标 (未使用)。
    /// * `_suns` - 阳光列表的引用 (未使用)。
    /// * `_peas` - 豌豆列表的引用 (未使用)。
    /// * `_zombies` - 僵尸列表的引用 (坚果墙的动作不依赖僵尸状态，故未使用)。
    fn update_action(&mut self, _grid_x: usize, _grid_y: usize, _suns: &mut Vec<Sun>, _peas: &mut Vec<Pea>, _zombies: &Vec<Zombie>) {
        // 坚果墙没有主动动作
    }

    /// 获取种植坚果墙所需的阳光花费。
    fn get_cost(&self) -> i32 {
        COST
    }

    /// 获取坚果墙在商店中显示的卡片图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    ///
    /// # Returns
    ///
    /// 返回坚果墙卡片图像的引用。
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image {
        &resources.wallnut_card
    }

    /// 获取坚果墙当前动画帧对应的图像。
    ///
    /// **注意**: 此默认实现可能需要根据 `Plant` 结构体中如何处理多损坏阶段图像来调整。
    /// 如果 `resources.wallnut_images` 存储的是所有损坏阶段的动画帧拼接，或者仅一个阶段的，
    /// 那么 `Plant::draw` 中可能需要更复杂的逻辑来选择正确的图像集和帧。
    /// 这里的实现是基于 `resources.wallnut_images` 包含当前应显示状态的动画帧。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `animation_frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回当前动画帧图像的引用。如果图像资源未加载，则返回卡片图像作为备用。
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a graphics::Image {
        // 注意：这里的实现假设 resources.wallnut_images 存储的是当前状态的动画帧
        // 如果坚果墙的不同损坏阶段使用不同的图像Vec，则Plant结构体中的draw逻辑需要更复杂处理
        let frame_count = resources.wallnut_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.wallnut_images[safe_index]
        } else {
            &resources.wallnut_card // Fallback
        }
    }

    /// 获取坚果墙的损坏状态数量。
    ///
    /// # Returns
    ///
    /// 返回坚果墙具有的视觉损坏阶段的数量 (`usize`)。
    fn get_damage_state(&self) -> usize {
        DAMAGE_STATE_COUNT
    }
}