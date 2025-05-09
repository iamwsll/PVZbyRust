// src/zombies/zombie_trait.rs
use crate::resources::Resources;
use ggez::{Context, GameResult};
use ggez::graphics::{Image, DrawParam};

/// 僵尸特性，定义所有僵尸需要实现的方法
pub trait ZombieTrait {
    /// 获取僵尸的初始生命值
    fn get_initial_health(&self) -> i32;
    
    /// 获取僵尸的移动速度
    fn get_speed(&self) -> f32;
    
    /// 获取僵尸的攻击伤害
    fn get_attack_damage(&self) -> i32;
    
    /// 获取僵尸的攻击间隔（毫秒）
    fn get_attack_interval(&self) -> u64;
    
    /// 获取僵尸的行走动画帧数
    fn get_walk_frame_count(&self) -> usize {
        // 默认返回22帧
        22
    }
    
    /// 获取僵尸的攻击动画帧数
    fn get_attack_frame_count(&self) -> usize {
        // 默认返回21帧
        21
    }
    
    /// 获取僵尸死亡动画帧数
    fn get_die_frame_count(&self) -> usize {
        // 默认返回10帧
        10
    }
    
    /// 获取僵尸行走动画图片
    fn get_walk_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let walk_frame_count = resources.zombies_walk1_images.len();
        if walk_frame_count > 0 {
            &resources.zombies_walk1_images[frame % walk_frame_count]
        } else {
            &resources.zombies_walk1_images[0]
        }
    }
    
    /// 获取僵尸攻击动画图片
    fn get_attack_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let attack_frame_count = resources.zombie_attack_images.len();
        if attack_frame_count > 0 {
            &resources.zombie_attack_images[frame % attack_frame_count]
        } else {
            &resources.zombie_attack_images[0]
        }
    }
    
    /// 获取僵尸死亡动画图片
    fn get_die_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        let die_frame_count = resources.zombie_die_images.len();
        if die_frame_count > 0 && frame < die_frame_count {
            &resources.zombie_die_images[frame]
        } else if !resources.zombie_die_images.is_empty() {
            &resources.zombie_die_images[resources.zombie_die_images.len() - 1]
        } else {
            &resources.zombies_walk1_images[0]
        }
    }
    
    /// 僵尸特定的更新逻辑（如果有的话）
    fn update_special(&mut self, _dt: u64) {
        // 默认实现为空，子类可以覆盖
    }
    
    /// 获取僵尸特定的绘制参数（如缩放、偏移等）
    fn get_draw_params(&self) -> DrawParam {
        // 默认绘制参数
        DrawParam::default().scale([0.8, 0.8])
    }
    
    /// 僵尸是否有特殊能力（如路障僵尸的路障被打掉等）
    fn has_special_ability(&self) -> bool {
        false
    }
    
    /// 处理僵尸受到伤害时的特殊逻辑（如路障掉落等）
    fn handle_damage(&mut self, _damage: i32) -> bool {
        // 默认实现直接返回false，表示没有特殊处理
        false
    }
}