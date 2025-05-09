use crate::grid::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X, GRID_START_Y};
use crate::sun::Sun;
use crate::pea::{Pea, PeaType};
use crate::plants::plant_trait::PlantTrait;
use crate::resources::Resources;
use ggez::graphics;

// 豌豆射手植物
pub struct Peashooter {
    // 可以添加豌豆射手特有的状态
    shoot_timer: u64,
}

impl Peashooter {
    pub fn new() -> Self {
        Peashooter {
            shoot_timer: 0,
        }
    }
}

// 豌豆射手常量
const INITIAL_HEALTH: i32 = 300;
const COOLDOWN: u64 = 1400; // 发射间隔为1.4秒
const COST: i32 = 100;

impl PlantTrait for Peashooter {
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    fn get_cooldown(&self) -> u64 {
        COOLDOWN
    }
    
    fn get_frame_count(&self) -> usize {
        13 // 豌豆射手动画有13帧
    }
    
    fn update_action(&mut self, grid_x: usize, grid_y: usize, _suns: &mut Vec<Sun>, peas: &mut Vec<Pea>) {
        // 计算豌豆射手的位置，用于确定豌豆的发射位置
        let x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH * 0.8;
        let y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT + GRID_CELL_HEIGHT * 0.3;
        
        // 创建一个新豌豆
        let new_pea = Pea::new(x, y, grid_y, PeaType::Normal);
        
        // 添加到豌豆列表中
        peas.push(new_pea);
        
        // 重置发射计时器
        self.shoot_timer = 0;
    }
    
    fn get_cost(&self) -> i32 {
        COST
    }
    
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image {
        &resources.peashooter_card
    }
    
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a graphics::Image {
        let frame_count = resources.peashooter_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.peashooter_images[safe_index]
        } else {
            // 如果没有图像，返回卡片
            &resources.peashooter_card
        }
    }
}