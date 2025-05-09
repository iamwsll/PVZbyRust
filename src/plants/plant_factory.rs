use std::rc::Rc;
use crate::plants::plant_trait::PlantTrait;
use crate::plants::peashooter::Peashooter;
use crate::plants::sunflower::Sunflower;
use crate::plants::wallnut::WallNut;

/// 植物类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
    // 在这里添加新的植物类型
    // CherryBomb,
    // PotatoMine,
    // SnowPea,
}

impl PlantType {
    /// 获取植物的阳光成本
    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::WallNut => 50,
            // 添加新植物的成本
            // PlantType::CherryBomb => 150,
            // PlantType::PotatoMine => 25,
            // PlantType::SnowPea => 175,
        }
    }
}

/// 植物工厂，用于创建各种植物实例
pub struct PlantFactory;

impl PlantFactory {
    /// 创建指定类型的植物
    pub fn create_plant(plant_type: PlantType) -> Box<dyn PlantTrait> {
        match plant_type {
            PlantType::Peashooter => Box::new(Peashooter::new()),
            PlantType::Sunflower => Box::new(Sunflower::new()),
            PlantType::WallNut => Box::new(WallNut::new()),
            // 在这里添加新的植物类型
            // PlantType::CherryBomb => Box::new(CherryBomb::new()),
            // PlantType::PotatoMine => Box::new(PotatoMine::new()),
            // PlantType::SnowPea => Box::new(SnowPea::new()),
        }
    }
}