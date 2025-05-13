//! # 僵尸工厂模块 (`zombie_factory`)
//!
//! 定义了 `ZombieFactory` 结构体和 `ZombieType` 枚举，
//! 用于根据指定的类型创建不同种类的僵尸实例。
//! 这是一个典型的工厂模式实现，旨在解耦僵尸的创建逻辑和使用逻辑。

use crate::zombies::zombie_trait::ZombieTrait;
use crate::zombies::normal_zombie::NormalZombie;
use crate::zombies::conehead_zombie::ConeheadZombie;
// use crate::zombies::buckethead_zombie::BucketHeadZombie; // 示例：未来可添加的铁桶僵尸

/// 枚举了游戏中所有可能的僵尸类型。
///
/// 此枚举用于 `ZombieFactory` 来决定创建哪种具体的僵尸实例。
/// 它也可能在游戏逻辑的其他部分用于区分不同类型的僵尸。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZombieType {
    /// 普通僵尸，基础的敌人单位。
    Normal,
    /// 路障僵尸，比普通僵尸更耐打。
    Conehead,
    // /// 示例：铁桶僵尸，具有更高防御力。
    // Buckethead,
    // /// 示例：橄榄球僵尸，移动速度快，生命值高。
    // FootballZombie,
    // /// 示例：铁栅门僵尸，持有铁栅门作为盾牌。
    // ScreenDoorZombie,
}

/// 僵尸工厂结构体。
///
/// `ZombieFactory` 提供一个静态方法 `create_zombie`，
/// 该方法接受一个 `ZombieType` 参数，并返回一个实现了 `ZombieTrait` 的
/// 动态分发对象 (`Box<dyn ZombieTrait>`)。
pub struct ZombieFactory;

impl ZombieFactory {
    /// 根据给定的 `ZombieType` 创建并返回一个僵尸实例。
    ///
    /// # Arguments
    ///
    /// * `zombie_type` - 要创建的僵尸的类型 (`ZombieType`)。
    ///
    /// # Returns
    ///
    /// 返回一个 `Box<dyn ZombieTrait>`，其中包含了新创建的具体僵尸对象。
    /// 如果请求的僵尸类型未在工厂中定义，则此方法会 panic（当前实现中）。
    /// 在更健壮的实现中，可能会返回 `Result` 或 `Option`。
    pub fn create_zombie(zombie_type: ZombieType) -> Box<dyn ZombieTrait> {
        match zombie_type {
            ZombieType::Normal => Box::new(NormalZombie::new()),
            // 以后添加更多僵尸类型
            ZombieType::Conehead => Box::new(ConeheadZombie::new()),
            // ZombieType::Buckethead => Box::new(BucketHeadZombie::new()),
        }
    }
}