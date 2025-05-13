//! # 僵尸特征模块 (`zombie_trait`)
//!
//! 定义了 `ZombieTrait`，这是一个所有具体僵尸类型都需要实现的特征（接口）。
//! 它规定了僵尸必须具备的一系列通用行为和属性获取方法，
//! 包括生命值、速度、攻击属性、动画帧数和图像资源等。
//! 许多方法提供了默认实现，特别是对于动画帧和图像，它们会从 `Resources`
//! 中获取通用的僵尸动画资源。具体僵尸类型可以覆盖这些方法以提供特有的行为或视觉效果。

use crate::core::resources::Resources;
// use ggez::{Context, GameResult}; // Context 和 GameResult 未在此 trait 的方法签名中直接使用
use ggez::graphics::{DrawParam, Image};

/// `ZombieTrait` 定义了所有僵尸类型共享的核心行为和属性。
///
/// 通过实现此特征，不同类型的僵尸（如普通僵尸、路障僵尸）可以被泛化处理，
/// 并由通用的 `Zombie` 结构体进行管理。
pub trait ZombieTrait {
    /// 获取僵尸的初始生命值。
    ///
    /// # Returns
    ///
    /// 返回僵尸的初始生命值 (`i32`)。
    fn get_initial_health(&self) -> i32;
    
    /// 获取僵尸的移动速度。
    ///
    /// # Returns
    ///
    /// 返回僵尸的移动速度，单位通常是像素/毫秒 (`f32`)。
    fn get_speed(&self) -> f32;
    
    /// 获取僵尸的攻击伤害值。
    ///
    /// # Returns
    ///
    /// 返回僵尸单次攻击造成的伤害 (`i32`)。
    fn get_attack_damage(&self) -> i32;
    
    /// 获取僵尸的攻击间隔。
    ///
    /// # Returns
    ///
    /// 返回两次攻击之间的最小时间间隔，单位为毫秒 (`u64`)。
    fn get_attack_interval(&self) -> u64;
    
    /// 获取僵尸行走动画的总帧数。
    ///
    /// 默认实现返回22帧。
    ///
    /// # Returns
    ///
    /// 返回行走动画的帧数 (`usize`)。
    fn get_walk_frame_count(&self) -> usize {
        // 默认返回22帧
        22
    }
    
    /// 获取僵尸攻击动画的总帧数。
    ///
    /// 默认实现返回21帧。
    ///
    /// # Returns
    ///
    /// 返回攻击动画的帧数 (`usize`)。
    fn get_attack_frame_count(&self) -> usize {
        // 默认返回21帧
        21
    }
    
    /// 获取僵尸死亡动画的总帧数。
    ///
    /// 默认实现返回10帧。
    ///
    /// # Returns
    ///
    /// 返回死亡动画的帧数 (`usize`)。
    fn get_die_frame_count(&self) -> usize {
        // 默认返回10帧
        10
    }
    
    /// 获取僵尸当前行走动画帧对应的图像。
    ///
    /// 默认实现从 `resources.zombies_walk1_images` 中获取图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回对应帧的图像引用 (`&'a Image`)。
    fn get_walk_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let walk_frame_count = resources.zombies_walk1_images.len();
        if walk_frame_count > 0 {
            &resources.zombies_walk1_images[frame % walk_frame_count]
        } else {
            &resources.zombies_walk1_images[0]
        }
    }
    
    /// 获取僵尸当前攻击动画帧对应的图像。
    ///
    /// 默认实现从 `resources.zombie_attack_images` 中获取图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回对应帧的图像引用 (`&'a Image`)。
    fn get_attack_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let attack_frame_count = resources.zombie_attack_images.len();
        if attack_frame_count > 0 {
            &resources.zombie_attack_images[frame % attack_frame_count]
        } else {
            &resources.zombie_attack_images[0]
        }
    }
    
    /// 获取僵尸当前死亡动画帧对应的图像。
    ///
    /// 默认实现从 `resources.zombie_die_images` 中获取图像。
    /// 如果请求的帧超出了动画范围，则返回最后一帧。
    /// 如果死亡动画图像列表为空，则回退到行走动画的第一帧作为备用。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回对应帧的图像引用 (`&'a Image`)。
    fn get_die_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let die_frame_count = resources.zombie_die_images.len();
        if die_frame_count > 0 && frame < die_frame_count {
            &resources.zombie_die_images[frame]
        } else if !resources.zombie_die_images.is_empty() {
            &resources.zombie_die_images[resources.zombie_die_images.len() - 1]
        } else {
            &resources.zombies_walk1_images[0]
        }
    }
    
    /// （可选）执行僵尸的特殊更新逻辑。
    ///
    /// 此方法每帧被调用，允许具体僵尸类型实现其特有的行为或状态更新。
    /// 默认实现为空。
    ///
    /// # Arguments
    ///
    /// * `_dt` - 自上一帧以来经过的时间（毫秒），默认未使用。
    fn update_special(&mut self, _dt: u64) {
        // 默认实现为空，子类可以覆盖
    }
    
    /// （可选）获取用于绘制此僵尸的特定 `DrawParam`。
    ///
    /// 这允许具体僵尸类型自定义其绘制时的缩放、旋转、颜色等参数。
    /// 默认实现返回一个将图像缩放为原始大小80%的 `DrawParam`。
    ///
    /// # Returns
    ///
    /// 返回一个 `DrawParam` 实例。
    fn get_draw_params(&self) -> DrawParam {
        // 默认绘制参数
        DrawParam::default().scale([0.8, 0.8])
    }
    
    /// （可选）指示此僵尸当前是否具有某种特殊能力或状态。
    ///
    /// 例如，路障僵尸在失去路障前具有特殊能力。
    /// 默认实现返回 `false`。
    ///
    /// # Returns
    ///
    /// 如果僵尸具有特殊能力，则返回 `true`。
    fn has_special_ability(&self) -> bool {
        false
    }
    
    /// （可选）处理僵尸受到伤害时的特殊逻辑。
    ///
    /// 例如，路障僵尸在受到一定伤害后会失去路障并转变形态。
    /// 此方法在通用伤害计算之前被调用。
    ///
    /// # Arguments
    ///
    /// * `_damage` - 对僵尸造成的伤害值，默认未使用。
    ///
    /// # Returns
    ///
    /// 如果此方法处理了伤害（例如，吸收了伤害或触发了状态转变），
    /// 则应返回 `true`，以阻止后续的通用伤害计算。
    /// 否则返回 `false`。
    fn handle_damage(&mut self, _damage: i32) -> bool {
        // 默认实现直接返回false，表示没有特殊处理
        false
    }
    
    /// （可选）在僵尸因特殊能力（如 `handle_damage`）而转变形态后，获取其新的生命值上限。
    ///
    /// 如果僵尸转变形态导致其最大生命值需要改变（例如，路障僵尸失去路障后，
    /// 其生命值上限应变为普通僵尸的生命值上限），此方法应返回 `Some(new_max_health)`。
    /// 如果形态转变不影响最大生命值，或没有发生形态转变，则返回 `None`。
    ///
    /// # Returns
    ///
    /// 返回 `Option<i32>`，其中 `Some(value)` 表示新的生命值上限，`None` 表示无变化。
    fn transform_health(&self) -> Option<i32> {
        // 默认实现返回None，表示没有变形
        None
    }
}