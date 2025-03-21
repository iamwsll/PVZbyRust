use bevy::prelude::*;
use crate::components::*;

pub fn collision_detection_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    mut zombie_query: Query<(Entity, &mut Zombie, &Transform)>,
    time: Res<Time>,
) {
    // 移动所有子弹
    for (projectile_entity, mut transform, projectile) in &mut projectile_query {
        transform.translation.x += projectile.speed * time.delta().as_secs_f32();
        
        // 如果子弹飞出屏幕，删除它
        if transform.translation.x > 450.0 {
            commands.entity(projectile_entity).despawn();
            continue;
        }
        
        // 检测子弹与僵尸的碰撞
        for (zombie_entity, mut zombie, zombie_transform) in &mut zombie_query {
            let distance = zombie_transform.translation.distance(transform.translation);
            
            // 简单碰撞检测
            if distance < 30.0 {
                // 对僵尸造成伤害
                zombie.health -= projectile.damage;
                
                // 如果僵尸生命值为0或以下，删除僵尸
                if zombie.health <= 0.0 {
                    commands.entity(zombie_entity).despawn();
                }
                
                // 碰撞后删除子弹
                commands.entity(projectile_entity).despawn();
                break;
            }
        }
    }
}
