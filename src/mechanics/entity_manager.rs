//! # 实体管理器模块 (`entity_manager`)
//!
//! 本模块负责在游戏过程中动态创建和管理各种游戏实体，
//! 例如自然生成的阳光和根据关卡进度出现的僵尸。
//! 它通常与 `LevelController` 协作来决定何时以及生成何种僵尸。

use crate::zombies::{Zombie, ZombieType};
use crate::entities::sun::{Sun, SunType};
use crate::mechanics::level_controller::{LevelController, ZombieSpawnInfo};
use rand::Rng;

/// 实体管理器结构体。
///
/// 包含一个 `LevelController` 实例，用于获取僵尸生成的信息。
pub struct EntityManager {
    /// 关卡控制器，用于决定何时以及生成何种类型的僵尸。
    pub level_controller: LevelController,
}

impl EntityManager {
    /// 创建一个新的 `EntityManager` 实例。
    ///
    /// 初始化时会创建一个新的 `LevelController`。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `EntityManager` 实例。
    pub fn new() -> Self {
        EntityManager {
            level_controller: LevelController::new(),
        }
    }

    /// 更新实体管理器状态，主要通过更新其内部的 `LevelController` 来获取需要生成的僵尸信息。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒），传递给 `LevelController`。
    ///
    /// # Returns
    ///
    /// 返回一个包含 `ZombieSpawnInfo` 的向量，指示需要生成的僵尸类型和行号。
    pub fn update(&mut self, dt: u64) -> Vec<ZombieSpawnInfo> {
        self.level_controller.update(dt)
    }

    /// 根据指定的类型和行号生成一个新的僵尸实例。
    ///
    /// # Arguments
    ///
    /// * `zombie_type` - 要生成的僵尸类型 (`ZombieType`)。
    /// * `row` - 僵尸生成的行索引。
    ///
    /// # Returns
    ///
    /// 返回一个新创建的 `Zombie` 实例。
    pub fn spawn_zombie(&self, zombie_type: ZombieType, row: usize) -> Zombie {
        Zombie::new(zombie_type, row)
    }

    /// 生成一个自然掉落的阳光实例。
    ///
    /// 阳光的x坐标在一定范围内随机生成，y坐标初始为0（屏幕顶端）。
    ///
    /// # Returns
    ///
    /// 返回一个新创建的 `Sun` 实例，其类型为 `SunType::NaturalGeneration`。
    pub fn spawn_natural_sun(&self) -> Sun {
        let x = rand::thread_rng().gen_range(50.0..750.0);
        Sun::new(x, 0.0, SunType::NaturalGeneration)
    }

    /// 决定当前是否应该生成一个自然阳光。
    ///
    /// 基于一个固定的随机概率（当前为每500次调用大约触发一次）。
    ///
    /// # Returns
    ///
    /// 如果随机条件满足，则返回 `true`，否则返回 `false`。
    pub fn should_spawn_natural_sun() -> bool {
        // 使用随机率决定是否生成阳光
        rand::random::<u32>() % 500 == 0
    }
}