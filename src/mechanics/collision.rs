//! # 碰撞检测模块 (`collision`)
//!
//! 本模块负责处理游戏中不同实体之间的碰撞检测及相应的逻辑响应。
//! 例如，豌豆与僵尸的碰撞会导致僵尸受伤和豌豆消失；僵尸与植物的碰撞会导致植物受损以及僵尸进入攻击状态。

use crate::entities::pea::Pea;
use crate::plants::Plant;
use crate::zombies::Zombie;
use ggez::{Context, timer};

/// 碰撞管理器结构体。
///
/// 这是一个无状态的工具结构体，提供了静态方法来处理不同类型的碰撞事件。
pub struct CollisionManager;

impl CollisionManager {
    /// 处理豌豆与僵尸之间的碰撞。
    ///
    /// 遍历所有活动的豌豆和未处于死亡动画中的僵尸：
    /// 1. 检查它们是否在同一行。
    /// 2. 检查豌豆的x坐标是否已达到或超过僵尸的x坐标（粗略检测）。
    /// 3. 如果满足以上条件，则获取两者的精确碰撞矩形并检查是否重叠。
    /// 4. 如果发生碰撞，僵尸受到伤害，豌豆被标记为非活动状态并记录下来。
    ///
    /// 完成遍历后，移除所有非活动的豌豆，并移除所有死亡动画已完成的僵尸。
    ///
    /// # Arguments
    ///
    /// * `peas` - 一个可变的豌豆向量引用，包含游戏中所有的豌豆。
    /// * `zombies` - 一个可变的僵尸向量引用，包含游戏中所有的僵尸。
    pub fn handle_pea_zombie_collision(peas: &mut Vec<Pea>, zombies: &mut Vec<Zombie>) {
        let mut inactive_peas = Vec::new();
    
        // 检测豌豆和僵尸的碰撞
        for (pea_idx, pea) in peas.iter_mut().enumerate() {
            if !pea.active {
                inactive_peas.push(pea_idx);
                continue;
            }
    
            for zombie in zombies.iter_mut() {
                // 如果僵尸已经在死亡动画中，跳过碰撞检测
                if zombie.is_dying {
                    continue;
                }
                
                // 如果不在同一行，跳过检测
                if pea.row != zombie.row {
                    continue;
                }
        
                // 如果豌豆位置超过僵尸位置，可能发生碰撞
                if pea.x + 20.0 >= zombie.x {
                    // 检查碰撞
                    let zombie_rect = zombie.get_rect();
                    let pea_rect = pea.get_rect();
            
                    if pea_rect.overlaps(&zombie_rect) {
                        // 碰撞发生，僵尸受伤
                        zombie.take_damage(pea.damage);
                
                        // 豌豆击中后消失
                        pea.active = false;
                        inactive_peas.push(pea_idx);
                        break; // 一个豌豆只能击中一个僵尸
                    }
                }
            }
        }
        
        // 移除已经无效的豌豆
        inactive_peas.sort_by(|a, b| b.cmp(a));
        for idx in inactive_peas {
            peas.remove(idx);
        }
        
        // 移除死亡动画已完成的僵尸
        zombies.retain(|zombie| !zombie.death_animation_complete);
    }

    /// 处理僵尸与植物之间的交互（主要是攻击）。
    ///
    /// 遍历所有未死亡的僵尸：
    /// 1. 对每个僵尸，遍历所有未死亡的植物。
    /// 2. 检查僵尸是否在其正前方（同一行且x坐标接近）遇到植物。
    /// 3. 如果遇到，则将僵尸设置为攻击状态，记录攻击目标，并使僵尸对植物造成伤害。
    /// 4. 检查植物在受到伤害后是否死亡，如果死亡则标记。
    /// 5. 一个僵尸同时只能攻击一个植物，一旦找到目标则停止对当前僵尸的植物搜索。
    ///
    /// 完成遍历后，更新所有僵尸的攻击状态，并移除所有已死亡的植物。
    ///
    /// # Arguments
    ///
    /// * `zombies` - 一个可变的僵尸向量引用。
    /// * `plants` - 一个可变的植物向量引用。
    /// * `ctx` - ggez的上下文环境，主要用于获取时间增量以计算攻击伤害。
    pub fn handle_zombie_plant_interaction(zombies: &mut Vec<Zombie>, plants: &mut Vec<Plant>, ctx: &mut Context) {
        // 遍历所有僵尸
        for zombie in zombies {
            // 如果僵尸已经死亡，跳过
            if zombie.is_dying {
                continue;
            }
            
            let mut is_attacking = false;
            let mut target_index = None;
            
            // 检查是否有植物在僵尸前方
            for (i, plant) in plants.iter_mut().enumerate() {
                // 如果植物已经死亡，跳过
                if plant.is_dead {
                    continue;
                }
                
                // 检查僵尸是否可以攻击这个植物
                if zombie.has_plant_in_front(plant.grid_x, plant.grid_y) {
                    // 设置僵尸为攻击状态
                    is_attacking = true;
                    target_index = Some(i);
                    
                    // 僵尸攻击植物
                    zombie.attack_plant(&mut plant.health, timer::delta(ctx).as_millis() as u64);
                    
                    // 检查植物是否死亡
                    if plant.health <= 0 {
                        plant.is_dead = true;
                    }
                    
                    break; // 一个僵尸同时只能攻击一个植物
                }
            }
            
            // 更新僵尸的攻击状态
            zombie.set_attacking(is_attacking, target_index);
        }
        
        // 移除死亡的植物
        plants.retain(|plant| !plant.is_dead);
    }
}