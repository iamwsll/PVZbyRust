use crate::entities::pea::Pea;
use crate::plants::Plant;
use crate::zombies::Zombie;
use ggez::{Context, timer};

/// 碰撞检测模块，负责处理游戏中的碰撞检测相关逻辑
pub struct CollisionManager;

impl CollisionManager {
    /// 处理豌豆和僵尸的碰撞
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

    /// 处理僵尸和植物的交互
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