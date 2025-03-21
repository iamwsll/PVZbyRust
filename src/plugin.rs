use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::*;
use crate::states::GameState;

pub struct PVZPlugin;

impl Plugin for PVZPlugin {
    fn build(&self, app: &mut App) {
        app
            // 注册资源
            .init_resource::<GameTextures>()
            .init_resource::<SunCounter>()
            
            // 添加启动系统
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, load_game_textures)
            
            // 添加菜单状态系统
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            
            // 添加游戏状态系统
            .add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(Update, spawn_zombie_system.run_if(in_state(GameState::InGame)))
            .add_systems(Update, zombie_movement_system.run_if(in_state(GameState::InGame)))
            .add_systems(Update, plant_shooting_system.run_if(in_state(GameState::InGame)))
            .add_systems(Update, collision_detection_system.run_if(in_state(GameState::InGame)))
            .add_systems(Update, sun_collection_system.run_if(in_state(GameState::InGame)))
            .add_systems(Update, plant_placement_system.run_if(in_state(GameState::InGame)))
            // 添加游戏结束状态系统
            .add_systems(OnEnter(GameState::GameOver), setup_game_over);
    }
}
