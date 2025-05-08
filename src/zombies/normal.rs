// src/zombies/normal.rs

// 普通僵尸的特定属性
pub const INITIAL_HEALTH: i32 = 200;
pub const SPEED: f32 = 0.02; // 每毫秒移动的像素
pub const ATTACK_DAMAGE: i32 = 50; // 每次攻击造成的伤害
pub const ATTACK_INTERVAL: u64 = 1000; // 攻击间隔为1秒
// 普通僵尸的特定更新逻辑 (如果需要)
pub fn update() {
    // 普通僵尸可能没有特殊的更新逻辑
}

// 普通僵尸的特定绘制逻辑 (如果需要，例如不同的动画)
// pub fn draw() { ... }
