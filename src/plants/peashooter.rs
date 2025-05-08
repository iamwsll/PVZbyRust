use crate::pea::{Pea, PeaType};
use crate::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH};

pub const INITIAL_HEALTH: i32 = 300;
pub const COOLDOWN: u64 = 1500; // 发射间隔为1.5秒

// 豌豆射手的特定更新逻辑
pub fn update(grid_x: usize, grid_y: usize, peas: &mut Vec<Pea>) {
    // 计算豌豆射手的位置，用于确定豌豆的发射位置
    let x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH * 0.7;
    let y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT + GRID_CELL_HEIGHT * 0.3;
    
    // 创建一个新豌豆
    let new_pea = Pea::new(x, y, grid_y, PeaType::Normal);
    
    // 添加到豌豆列表中
    peas.push(new_pea);
}