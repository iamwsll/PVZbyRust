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
/// 同时跟踪已掉落的阳光数量和下一次自然阳光生成的时间。
pub struct EntityManager {
    /// 关卡控制器，用于决定何时以及生成何种类型的僵尸。
    pub level_controller: LevelController,
    
    /// 已生成的自然阳光数量
    fallen_sun_count: u32,
    
    /// 下一次自然阳光生成的倒计时（厘秒）
    next_sun_countdown: u32,
}

impl EntityManager {
    /// 创建一个新的 `EntityManager` 实例。
    ///
    /// 初始化时会创建一个新的 `LevelController`，
    /// 并设置初始阳光生成时间。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `EntityManager` 实例。
    pub fn new() -> Self {
        // 初始阳光倒计时设置为 425 + 随机数(0-275)厘秒
        let initial_countdown = 425 + rand::thread_rng().gen_range(0..=275);
        
        EntityManager {
            level_controller: LevelController::new(),
            fallen_sun_count: 0,
            next_sun_countdown: initial_countdown,
        }
    }

    /// 更新实体管理器状态，主要通过更新其内部的 `LevelController` 来获取需要生成的僵尸信息。
    /// 同时也负责更新自然阳光的生成计时器。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒），传递给 `LevelController`。
    ///
    /// # Returns
    ///
    /// 返回一个包含 `ZombieSpawnInfo` 的向量，指示需要生成的僵尸类型和行号。
    /// 注意：自然阳光的生成需要通过调用 `check_natural_sun_spawn` 方法来检查。
    pub fn update(&mut self, dt: u64) -> Vec<ZombieSpawnInfo> {
        // 更新关卡控制器获取僵尸生成信息
        self.level_controller.update(dt)
    }
    
    /// 检查并更新自然阳光生成倒计时。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒）。
    ///
    /// # Returns
    ///
    /// 如果当前应该生成阳光，则返回 `true`，否则返回 `false`。
    pub fn check_natural_sun_spawn(&mut self, dt: u64) -> bool {
        // 将毫秒转换为厘秒
        let dt_centiseconds = dt * 100 / 1000;
        self.should_spawn_natural_sun(dt_centiseconds)
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
    /// 阳光的x y坐标在一定范围内随机生成。
    ///
    /// # Returns
    ///
    /// 返回一个新创建的 `Sun` 实例，其类型为 `SunType::NaturalGeneration`。
    pub fn spawn_natural_sun(&self) -> Sun {
        let x = rand::thread_rng().gen_range(50.0..750.0);
        let y = rand::thread_rng().gen_range(30.0..100.0);
        Sun::new(x, y, SunType::NaturalGeneration)
    }

    /// 决定当前是否应该生成一个自然阳光。
    ///
    /// 基于时间倒计时系统决定是否应该生成阳光。
    /// 每当倒计时结束时，生成一个自然阳光，并重置倒计时。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（厘秒）。
    ///
    /// # Returns
    ///
    /// 如果当前应该生成阳光，则返回 `true`，否则返回 `false`。
    pub fn should_spawn_natural_sun(&mut self, dt: u64) -> bool {
        if self.next_sun_countdown <= dt as u32 {
            // 倒计时结束，应当生成阳光
            self.fallen_sun_count += 1;
            
            // 计算新的等待时间
            // min{已掉落阳光数量 × 10 + 425, 950} + rand(0, 275)
            let base_time = std::cmp::min(self.fallen_sun_count * 10 + 425, 950);
            let random_addition = rand::thread_rng().gen_range(0..=275);
            self.next_sun_countdown = base_time + random_addition;
            
            true
        } else {
            // 减少倒计时
            self.next_sun_countdown -= dt as u32;
            false
        }
    }
}