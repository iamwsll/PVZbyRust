use crate::entities::sun::Sun;
use crate::entities::pea::Pea;
use crate::plants::plant_trait::PlantTrait;
use crate::core::resources::Resources;
use ggez::graphics::Image;

// 坚果墙植物
pub struct WallNut;

impl WallNut {
    pub fn new() -> Self {
        WallNut
    }
}

// 坚果墙常量
const INITIAL_HEALTH: i32 = 4000; // 坚果墙有较高的生命值
const COST: i32 = 50;

impl PlantTrait for WallNut {
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    fn get_cooldown(&self) -> u64 {
        0 // 坚果墙没有主动行为，所以冷却时间为0
    }
    
    fn get_frame_count(&self) -> usize {
        16 // 坚果墙动画有16帧
    }
    
    fn update_action(&mut self, _grid_x: usize, _grid_y: usize, _suns: &mut Vec<Sun>, _peas: &mut Vec<Pea>) {
        // 坚果墙没有主动行为，所以不需要在这里做任何事情
    }
    
    fn get_cost(&self) -> i32 {
        COST
    }
    
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a Image {
        &resources.wallnut_card
    }
    
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a Image {
        let frame_count = resources.wallnut_images.len();
        if frame_count > 0 {
            let safe_index = animation_frame % frame_count;
            &resources.wallnut_images[safe_index]
        } else {
            // 如果没有图像，返回卡片
            &resources.wallnut_card
        }
    }
    
    // 坚果墙特有的损坏状态功能
    fn get_damage_state(&self, health: i32, max_health: i32) -> usize {
        // 根据生命值百分比返回损坏状态
        // 0: 完好 (100%-66%)
        // 1: 轻微损坏 (66%-33%)
        // 2: 严重损坏 (33%-0%)
        let health_percent = (health as f32 / max_health as f32) * 100.0;
        if health_percent > 66.0 {
            0
        } else if health_percent > 33.0 {
            1
        } else {
            2
        }
    }
}