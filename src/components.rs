use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Plant {
    pub plant_type: PlantType,
    pub health: f32,
    pub cost: u32,
    pub cooldown: Timer,
}

#[derive(Clone, Copy, Hash,PartialEq, Eq)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
    CherryBomb,
}

impl PlantType {
    pub fn cost(&self) -> u32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::WallNut => 50,
            PlantType::CherryBomb => 150,
        }
    }
}

#[derive(Component)]
pub struct Zombie {
    pub zombie_type: ZombieType,
    pub health: f32,
    pub speed: f32,
}

#[derive(Clone, Copy, Hash,PartialEq, Eq)]
pub enum ZombieType {
    Regular,
    ConeHead,
    BucketHead,
}

#[derive(Component)]
pub struct Sun {
    pub value: u32,
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Card {
    pub plant_type: PlantType,
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct PlantSelector;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct GameOverMenu;
