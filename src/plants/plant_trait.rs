use crate::core::resources::Resources;
use crate::entities::sun::Sun;
use crate::entities::pea::Pea;
use ggez::{GameResult, Context, graphics::Image};

/// 植物特性，定义所有植物需要实现的方法
pub trait PlantTrait {
    /// 获取植物的初始生命值
    fn get_initial_health(&self) -> i32;
    
    /// 获取植物的冷却时间（如发射间隔、产生阳光间隔等）
    fn get_cooldown(&self) -> u64;
    
    /// 获取植物的动画帧数
    fn get_frame_count(&self) -> usize;
    
    /// 植物特定的更新逻辑
    fn update_action(&mut self, grid_x: usize, grid_y: usize, suns: &mut Vec<Sun>, peas: &mut Vec<Pea>);
    
    /// 获取植物的成本（阳光数量）
    fn get_cost(&self) -> i32;
    
    /// 获取植物的卡片图像
    fn get_card_image<'a>(&self, resources: &'a Resources) -> &'a Image;
    
    /// 获取当前植物动画帧对应的图像
    fn get_current_frame_image<'a>(&self, resources: &'a Resources, animation_frame: usize) -> &'a Image;
    
    /// 植物的损坏状态（如坚果墙的不同损坏阶段）
    fn get_damage_state(&self, health: i32, max_health: i32) -> usize {
        // 默认实现返回0（无损坏）
        0
    }
    
    /// 植物的特殊效果（如爆炸等），返回true表示触发了特效
    fn special_effect(&mut self, _grid_x: usize, _grid_y: usize) -> bool {
        // 默认实现无特殊效果
        false
    }
}