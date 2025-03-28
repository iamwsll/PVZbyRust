use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::GameState;
use rand::prelude::*;
use std::time::Duration;

pub fn spawn_zombie_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    time: Res<Time>,
    mut spawn_timer: Local<Option<Timer>>,
) {
    // 初始化计时器
    if spawn_timer.is_none() {
        *spawn_timer = Some(Timer::from_seconds(5.0, TimerMode::Repeating));
    }
    
    // 更新计时器
    if let Some(timer) = spawn_timer.as_mut() {
        timer.tick(time.delta());
        
        // 检查是否到达生成时间
        if timer.just_finished() {
            // 随机选择一个行
            let mut rng = rand::thread_rng();
            let row = rng.gen_range(0..5);
            
            // 随机选择一个僵尸类型
            let zombie_types = [ZombieType::Regular, ZombieType::ConeHead, ZombieType::BucketHead];
            let zombie_type = zombie_types[rng.gen_range(0..zombie_types.len())];
            
            // // 生成僵尸
            // commands.spawn((
            //     Sprite {
            //         texture: game_textures.zombies.get(&zombie_type).unwrap().clone(),
            //         transform: Transform::from_xyz(450.0, 250.0 - row as f32 * 100.0, 1.0),
            //         ..default()
            //     },
            //     Zombie {
            //         zombie_type,
            //         health: zombie_type.health(),
            //         speed: zombie_type.speed(),
            //     },
            //     GridPosition {
            //         x: 9, // 最右侧
            //         y: row,
            //     },
            // ));
        }
    }
}

pub fn zombie_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut GridPosition, &Zombie)>,
    plant_query: Query<(Entity, &GridPosition), With<Plant>>,
    mut game_grid: ResMut<GameGrid>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (zombie_entity, mut transform, mut grid_pos, zombie) in &mut query {
        // 移动僵尸
        // transform.translation.x -= zombie.speed * time.delta_seconds();
        
        // 更新网格位置
        let new_grid_x = ((transform.translation.x + 400.0) / 80.0).floor() as usize;
        
        // 检查是否到达左侧边界
        if transform.translation.x < -400.0 {
            // 僵尸到达，游戏结束
            game_state.set(GameState::GameOver);
            continue;
        }
        
        // 如果网格位置发生变化
        if new_grid_x != grid_pos.x && new_grid_x < 9 {
            // 更新网格位置
            grid_pos.x = new_grid_x;
            
            // 检查是否与植物碰撞
            for (plant_entity, plant_grid) in &plant_query {
                if plant_grid.x == grid_pos.x && plant_grid.y == grid_pos.y {
                    // 与植物碰撞，僵尸停止移动并攻击植物
                    commands.entity(plant_entity).despawn();
                    game_grid.grid[plant_grid.x][plant_grid.y] = None;
                    break;
                }
            }
        }
    }
}
