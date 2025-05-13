//! # 关卡控制器模块 (`level_controller`)
//!
//! 本模块负责管理游戏的关卡流程，特别是僵尸的生成逻辑。
//! 它决定了何时、何种类型以及在哪一行生成僵尸，从而控制游戏的难度和节奏。

use crate::zombies::ZombieType;
use std::time::{Duration, Instant};
use rand::Rng;
use crate::ui::grid::GRID_HEIGHT;

/// 定义了生成僵尸时所需的信息。
///
/// 包含要生成的僵尸类型和其出现的行号。
pub struct ZombieSpawnInfo {
    /// 要生成的僵尸的类型。
    pub zombie_type: ZombieType,
    /// 僵尸将要生成的行索引。
    pub row: usize,
}

/// 关卡控制器结构体。
///
/// 管理僵尸的生成时机和类型。它使用 `last_spawn_time` 和 `spawn_interval`
/// 来控制僵尸生成的时间间隔。
/// 未来可以扩展此结构体以支持更复杂的关卡逻辑，如波数、特定事件等。
pub struct LevelController {
    /// 上一次生成僵尸的时间点。
    last_spawn_time: Instant,
    /// 生成僵尸之间的时间间隔。
    spawn_interval: Duration,
    // TODO: 可以添加更多字段来控制关卡进度，例如波数、难度等
}

impl LevelController {
    /// 创建一个新的 `LevelController` 实例。
    ///
    /// 初始化时，设置上一次生成时间为当前时间，并设定一个初始的生成间隔。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `LevelController` 实例。
    pub fn new() -> Self {
        LevelController {
            last_spawn_time: Instant::now(),
            spawn_interval: Duration::from_secs(10), // 初始生成间隔
        }
    }

    /// 更新关卡控制器的状态，并决定是否需要生成新的僵尸。
    ///
    /// 如果当前时间与上次生成僵尸的时间间隔超过了 `spawn_interval`，
    /// 则会创建一个新的 `ZombieSpawnInfo`（目前固定生成路障僵尸在随机行），
    /// 并重置 `last_spawn_time`。
    ///
    /// # Arguments
    ///
    /// * `_dt` - 自上次更新以来的时间增量（毫秒）。当前未使用，但可用于未来动态调整生成逻辑。
    ///
    /// # Returns
    ///
    /// 返回一个 `Vec<ZombieSpawnInfo>`，其中包含所有在本次更新中需要生成的僵尸的信息。
    /// 如果不需要生成僵尸，则返回空向量。
    pub fn update(&mut self, _dt: u64) -> Vec<ZombieSpawnInfo> {
        let mut spawns = Vec::new();
        let now = Instant::now();

        if now.duration_since(self.last_spawn_time) >= self.spawn_interval {
            // TODO: 实现更复杂的生成逻辑，例如基于波数、时间调整类型和数量
            let mut rng = rand::thread_rng();
            let row = rng.gen_range(0..GRID_HEIGHT);
            let zombie_type = ZombieType::Conehead; // 目前只生成普通僵尸

            spawns.push(ZombieSpawnInfo { zombie_type, row });

            self.last_spawn_time = now; // 重置计时器
            // 可以动态调整 spawn_interval 以增加难度
            // self.spawn_interval = Duration::from_secs(rng.gen_range(5..12));
        }

        spawns
    }
}
