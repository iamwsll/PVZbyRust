// src/zombies/zombie_factory.rs
use crate::zombies::zombie_trait::ZombieTrait;
use crate::zombies::normal_zombie::NormalZombie;
// 在这里导入其他僵尸类型
use crate::zombies::conehead_zombie::ConeheadZombie;
// use crate::zombies::buckethead_zombie::BucketHeadZombie;

/// 僵尸类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZombieType {
    Normal,
    // 以后添加更多僵尸类型
    Conehead,
    // Buckethead,
    // FootballZombie,
    // ScreenDoorZombie,
}

/// 僵尸工厂，用于创建各种僵尸实例
pub struct ZombieFactory;

impl ZombieFactory {
    /// 创建指定类型的僵尸
    pub fn create_zombie(zombie_type: ZombieType) -> Box<dyn ZombieTrait> {
        match zombie_type {
            ZombieType::Normal => Box::new(NormalZombie::new()),
            // 以后添加更多僵尸类型
            ZombieType::Conehead => Box::new(ConeheadZombie::new()),
            // ZombieType::Buckethead => Box::new(BucketHeadZombie::new()),
        }
    }
}