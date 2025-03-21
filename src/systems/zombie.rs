use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::GameState;
use rand::prelude::*;
use std::time::Duration;

pub fn spawn_zombie_system(
    mut commands: Commands,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
) {
    // 创建一个计时器作为资源或组件来控制僵尸生成速度
    // 这里简化处理，每隔一段时间随机生成僵尸
    let mut rng = rand::thread_rng();
    
    if rng.gen_bool(0.005) {  // 每帧有0.5%的几率生成僵尸
        // 随机选择行
        let lane = rng.gen_range(0..5);
        
        // 随机选择僵尸类型
        let zombie_type = match rng.gen_range(0..3) {
            0 => ZombieType::Regular,
            1 => ZombieType::ConeHead,
            _ => ZombieType::BucketHead,
        };
        
        // 根据僵尸类型设置不同的生命值
        let health = match zombie_type {
            ZombieType::Regular => 100.0,
            ZombieType::ConeHead => 200.0,
            ZombieType::BucketHead => 300.0,
        };
        
        // 生成僵尸
        commands.spawn((
            SpriteBundle {
                texture: game_textures.zombies.get(&zombie_type).unwrap().clone(),
                transform: Transform::from_xyz(400.0, 250.0 - lane as f32 * 100.0, 1.0),
                ..default()
            },
            Zombie {
                zombie_type,
                health,
                speed: 20.0,
            },
            GridPosition {
                x: 9,  // 开始在最右侧
                y: lane,
            },
        ));
    }
}

pub fn zombie_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut zombie_query: Query<(Entity, &mut Transform, &mut GridPosition, &Zombie)>,
    plant_query: Query<(Entity, &GridPosition), With<Plant>>,
    mut game_grid: ResMut<GameGrid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (zombie_entity, mut zombie_transform, mut zombie_grid, zombie) in &mut zombie_query {
        // 检查当前网格位置是否有植物
        let mut blocked = false;
        
        for (plant_entity, plant_grid) in &plant_query {
            if plant_grid.x == zombie_grid.x - 1 && plant_grid.y == zombie_grid.y {
                blocked = true;
                // 这里应添加僵尸攻击植物的逻辑
                break;
            }
        }
        
        if !blocked {
            // 向左移动
            zombie_transform.translation.x -= zombie.speed * time.delta_seconds();
            
            // 更新网格位置（当僵尸移动到新网格时）
            let new_grid_x = ((zombie_transform.translation.x + 400.0) / 80.0) as usize;
            if new_grid_x < zombie_grid.x {
                zombie_grid.x = new_grid_x;
            }
        }
        
        // 检查僵尸是否到达了左侧边缘（游戏结束条件）
        if zombie_transform.translation.x <= -400.0 {
            next_state.set(GameState::GameOver);
        }
    }
}
