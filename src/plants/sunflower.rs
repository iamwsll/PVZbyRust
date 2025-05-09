use crate::ui::grid::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X, GRID_START_Y};
use crate::entities::sun::{Sun, SunType};
use crate::entities::pea::Pea;
use crate::plants::plant_trait::PlantTrait;
use crate::core::resources::Resources;
use ggez::graphics;

/// 向日葵植物
pub struct Sunflower {
    // 可以添加向日葵特有的状态
    sun_production_timer: u64,
}

impl Sunflower {
    pub fn new() -> Self {
        Sunflower {
            sun_production_timer: 0,
        }
    }
}

// 向日葵常量
const INITIAL_HEALTH: i32 = 300;
const COOLDOWN: u64 = 24000; // 太阳产生间隔，24秒
const COST: i32 = 50;

impl PlantTrait for Sunflower {
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    fn get_cooldown(&self) -> u64 {
        COOLDOWN
    }
    
    fn get_frame_count(&self) -> usize {
        18 // 向日葵动画有18帧
    }
    
    fn update_action(&mut self, grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>, _peas: &mut Vec<Pea>) {
        // 计算阳光生成的位置 (在向日葵上方一点)
        let sun_x = GRID_START_X + (grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 2.0;
        let sun_y = GRID_START_Y + (grid_y as f32) * GRID_CELL_HEIGHT; 

        // 创建新的阳光
        suns.push(Sun::new(sun_x, sun_y, SunType::SunflowerGeneration));
        
        // 重置计时器
        self.sun_production_timer = 0;
    }
    
    fn get_cost(&self) -> i32 {
        COST
    }
    
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a graphics::Image {
        &resources.sunflower_card
    }
    
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a graphics::Image {
        let frame_count = resources.sunflower_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.sunflower_images[safe_index]
        } else {
            // 如果没有图像，返回卡片
            &resources.sunflower_card
        }
    }
}
