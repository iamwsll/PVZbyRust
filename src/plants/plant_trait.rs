//! # 植物特征模块 (`plant_trait`)
//!
//! 定义了 `PlantTrait`，这是一个所有具体植物类型都需要实现的特征（接口）。
//! 它规定了植物必须具备的一系列通用行为和属性获取方法，
//! 使得游戏主逻辑可以通过统一的接口与不同类型的植物进行交互。

use crate::entities::sun::Sun;
use crate::entities::pea::Pea;
use crate::core::resources::Resources;
use ggez::graphics;
use crate::zombies::Zombie; // 确保 Zombie 类型已导入

/// `PlantTrait` 定义了所有植物共享的核心行为和属性。
///
/// 通过实现此特征，不同类型的植物（如豌豆射手、向日葵）可以被泛化处理。
pub trait PlantTrait {
    /// 获取植物的初始生命值。
    ///
    /// # Returns
    ///
    /// 返回植物的初始生命值 (`i32`)。
    fn get_initial_health(&self) -> i32;

    /// 获取植物动作的冷却时间（例如，攻击或产生阳光的间隔）。
    ///
    /// # Returns
    ///
    /// 返回冷却时间，单位为毫秒 (`u64`)。
    fn get_cooldown(&self) -> u64;

    /// 获取植物动画的总帧数。
    ///
    /// # Returns
    ///
    /// 返回动画的帧数 (`usize`)。
    fn get_frame_count(&self) -> usize;

    /// 更新植物的特定动作，例如发射豌豆或产生阳光。
    ///
    /// 此方法在植物的冷却计时器完成后被调用。
    ///
    /// # Arguments
    ///
    /// * `grid_x` - 植物所在的网格x坐标。
    /// * `grid_y` - 植物所在的网格y坐标。
    /// * `suns` - 一个可变向量的引用，用于收集新产生的阳光。
    /// * `peas` - 一个可变向量的引用，用于收集新发射的豌豆。
    /// * `zombies` - 一个僵尸向量的引用，用于检查僵尸位置。
    fn update_action(&mut self, grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>, peas: &mut Vec<Pea>, zombies: &Vec<Zombie>);
    
    /// 获取种植该植物所需的阳光成本。
    ///
    /// # Returns
    ///
    /// 返回阳光成本 (`i32`)。
    fn get_cost(&self) -> i32;

    /// 获取植物在商店中显示的卡片图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    ///
    /// # Returns
    ///
    /// 返回商店卡片图像的引用 (`&'a graphics::Image`)。
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image;

    /// 获取植物当前动画帧对应的图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `animation_frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回当前动画帧图像的引用 (`&'a graphics::Image`)。
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a graphics::Image;

    /// （可选）执行植物的特殊效果或被动能力。
    ///
    /// 默认实现为空。具体植物可以覆盖此方法以实现特殊逻辑。
    ///
    /// # Arguments
    ///
    /// * `_grid_x` - 植物所在的网格x坐标 (默认未使用)。
    /// * `_grid_y` - 植物所在的网格y坐标 (默认未使用)。
    fn special_effect(&mut self, _grid_x: usize, _grid_y: usize) {
        // 默认无特殊效果
    }

    /// （可选）获取植物的损坏状态数量（例如坚果墙的不同损坏外观）。
    ///
    /// 默认返回1，表示只有一个状态（未损坏）。
    ///
    /// # Returns
    ///
    /// 返回损坏状态的数量 (`usize`)。
    fn get_damage_state(&self) -> usize {
        1 // 默认只有一个状态（未损坏）
    }
}