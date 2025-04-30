use crate::sun::Sun; // 确保导入 Sun (虽然豌豆射手不产生)

pub const INITIAL_HEALTH: i32 = 300;
pub const COOLDOWN: u64 = 1500; // Example cooldown in milliseconds

// 豌豆射手的特定更新逻辑
// 接受一个可变的 suns 向量引用，但未使用
pub fn update(_grid_x: usize, _grid_y: usize) {
    // TODO: 实现发射豌豆的逻辑
    println!("Peashooter ready to shoot!"); // Placeholder
    // 不需要返回 None
}