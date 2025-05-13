//! # 植物工厂模块 (`plant_factory`)
//!
//! 本模块定义了 `PlantType` 枚举和 `PlantFactory` 结构体，
//! 用于根据指定的植物类型创建相应的植物实例 (`Box<dyn PlantTrait>`)。
//! 这种工厂模式使得在游戏逻辑中创建不同植物更加方便和类型安全。

use std::rc::Rc;
use crate::plants::plant_trait::PlantTrait;
use crate::plants::peashooter::Peashooter;
use crate::plants::sunflower::Sunflower;
use crate::plants::wallnut::WallNut;

/// 枚举了游戏中所有可用的植物类型。
///
/// 每种植物类型对应一个具体的植物实现。
/// 此枚举也用于确定植物的成本等属性。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlantType {
    /// 豌豆射手：基础远程攻击单位。
    Peashooter,
    /// 向日葵：产生阳光的资源单位。
    Sunflower,
    /// 坚果墙：高生命值的防御单位。
    WallNut,
    // 在这里添加新的植物类型
    // CherryBomb, // 樱桃炸弹 (示例)
    // PotatoMine, // 土豆地雷 (示例)
    // SnowPea,    // 寒冰射手 (示例)
}

impl PlantType {
    /// 获取指定植物类型种植所需的阳光成本。
    ///
    /// # Returns
    ///
    /// 返回该植物类型的阳光成本 (`i32`)。
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

/// 植物工厂结构体。
///
/// 提供一个静态方法 `create_plant` 来实例化不同类型的植物。
/// 这是一个无状态的工具结构体。
pub struct PlantFactory;

impl PlantFactory {
    /// 根据给定的 `PlantType` 创建并返回一个具体的植物实例。
    ///
    /// 植物实例被封装在 `Box<dyn PlantTrait>` 中，以实现动态分派。
    ///
    /// # Arguments
    ///
    /// * `plant_type` - 要创建的植物的类型 (`PlantType`)。
    ///
    /// # Returns
    ///
    /// 返回一个实现了 `PlantTrait` 的植物对象的 `Box` 指针。
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