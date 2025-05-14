//! # 普通僵尸模块 (`normal_zombie`)
//!
//! 实现了游戏中最基础的敌人单位——普通僵尸。
//! 普通僵尸具有标准的生命值、移动速度和攻击属性，
//! 是玩家在游戏初期就会遇到的主要威胁。

use crate::zombies::zombie_trait::ZombieTrait;
// use ggez::graphics::{Image, DrawParam}; // Image 和 DrawParam 未在此文件中直接使用，ZombieTrait 的方法签名负责

/// 普通僵尸的初始生命值。
pub const INITIAL_HEALTH: i32 = 200;
/// 普通僵尸的移动速度（像素/毫秒）。
pub const SPEED: f32 = 0.017; 
/// 普通僵尸的攻击伤害值。
pub const ATTACK_DAMAGE: i32 = 300; 
/// 普通僵尸的攻击间隔（毫秒）。
pub const ATTACK_INTERVAL: u64 = 1000; // 攻击间隔为1秒

/// 普通僵尸的结构体实现。
///
/// 这是一个单元结构体，因为普通僵尸的所有特定行为和属性
/// 都通过 `ZombieTrait` 的方法（部分是默认实现，部分是这里覆盖的常量）来定义。
pub struct NormalZombie;

impl NormalZombie {
    /// 创建一个新的 `NormalZombie` 实例。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `NormalZombie`。
    pub fn new() -> Self {
        NormalZombie
    }
}

impl ZombieTrait for NormalZombie {
    /// 获取普通僵尸的初始生命值。
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    /// 获取普通僵尸的移动速度。
    fn get_speed(&self) -> f32 {
        SPEED
    }
    
    /// 获取普通僵尸的攻击伤害。
    fn get_attack_damage(&self) -> i32 {
        ATTACK_DAMAGE
    }
    
    /// 获取普通僵尸的攻击间隔。
    fn get_attack_interval(&self) -> u64 {
        ATTACK_INTERVAL
    }
    
    // 普通僵尸使用 `ZombieTrait` 中定义的默认实现来获取动画帧数和图像，
    // 以及处理特殊能力和伤害。
}