// src/zombies/normal_zombie.rs
use crate::zombies::zombie_trait::ZombieTrait;
use ggez::graphics::{Image, DrawParam};
use crate::core::resources::Resources;

// 普通僵尸的特定属性
const INITIAL_HEALTH: i32 = 200;
const SPEED: f32 = 0.02; // 每毫秒移动的像素
const ATTACK_DAMAGE: i32 = 50; // 每次攻击造成的伤害
const ATTACK_INTERVAL: u64 = 1000; // 攻击间隔为1秒

/// 普通僵尸的具体实现
pub struct NormalZombie;

impl NormalZombie {
    pub fn new() -> Self {
        NormalZombie
    }
}

impl ZombieTrait for NormalZombie {
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    fn get_speed(&self) -> f32 {
        SPEED
    }
    
    fn get_attack_damage(&self) -> i32 {
        ATTACK_DAMAGE
    }
    
    fn get_attack_interval(&self) -> u64 {
        ATTACK_INTERVAL
    }
    
    // 普通僵尸使用默认实现的其他方法
}