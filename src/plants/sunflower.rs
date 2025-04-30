use crate::grid::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X, GRID_START_Y};
use crate::sun::{Sun, SunType};

pub const INITIAL_HEALTH: i32 = 300;
pub const COOLDOWN: u64 = 10000; // Example cooldown for producing sun

// 向日葵的特定更新逻辑
// 接受一个可变的 suns 向量引用
pub fn update(grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>) {
    // 增加随机性：例如， 按概率产生阳光
    // if rand::random::<f32>() < 0.05 {
        update_sun(grid_x, grid_y, suns); // 产生阳光
    // }
}

// 接受一个可变的 suns 向量引用
fn update_sun(grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>) {
    // 计算阳光生成的位置 (在向日葵上方一点)
    let sun_x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 2.0;
    let sun_y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT; // 稍微向上偏移
    // 直接将新阳光添加到传入的向量中
    suns.push(Sun::new(sun_x, sun_y, SunType::SunflowerGeneration));
}
