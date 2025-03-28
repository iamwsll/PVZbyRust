use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::{PlantType, ZombieType};

#[derive(Resource, Default)]
pub struct GameTextures {
    pub plants: HashMap<PlantType, Handle<Image>>,
    pub zombies: HashMap<ZombieType, Handle<Image>>,
    pub backgrounds: HashMap<String, Handle<Image>>,
    pub sun: Handle<Image>,
    pub projectile: Handle<Image>,
    pub cards: HashMap<PlantType, Handle<Image>>,
}

#[derive(Resource)]
pub struct SunCounter {
    pub value: u32,
}

impl Default for SunCounter {
    fn default() -> Self {
        Self { value: 50 }
    }
}

// GameGrid资源用于存储游戏网格的信息
#[derive(Resource)]
pub struct GameGrid {
    pub grid: [[Option<Entity>; 5]; 9],
    pub cell_size: Vec2,
}

impl Default for GameGrid {
    fn default() -> Self {
        Self {
            grid: [[None; 5]; 9],
            cell_size: Vec2::new(80.0, 100.0),
        }
    }
}
