use crate::zombies::ZombieType;
use std::time::{Duration, Instant};
use rand::Rng;
use crate::grid::GRID_HEIGHT;

// 定义僵尸生成信息
pub struct ZombieSpawnInfo {
    pub zombie_type: ZombieType,
    pub row: usize,
}

pub struct LevelController {
    last_spawn_time: Instant,
    spawn_interval: Duration,
    // 可以添加更多字段来控制关卡进度，例如波数、难度等
}

impl LevelController {
    pub fn new() -> Self {
        LevelController {
            last_spawn_time: Instant::now(),
            spawn_interval: Duration::from_secs(10), // 初始生成间隔
        }
    }

    /// 更新控制器状态，返回需要生成的僵尸信息列表
    /// @param dt 时间增量（毫秒）
    /// @return Vec<ZombieSpawnInfo> 需要生成的僵尸信息列表
    /// @note 这里的 dt 可以用于动态调整生成间隔或其他逻辑
    pub fn update(&mut self, _dt: u64) -> Vec<ZombieSpawnInfo> {
        let mut spawns = Vec::new();
        let now = Instant::now();

        if now.duration_since(self.last_spawn_time) >= self.spawn_interval {
            // TODO: 实现更复杂的生成逻辑，例如基于波数、时间调整类型和数量
            let mut rng = rand::thread_rng();
            let row = rng.gen_range(0..GRID_HEIGHT);
            let zombie_type = ZombieType::Normal; // 目前只生成普通僵尸

            spawns.push(ZombieSpawnInfo { zombie_type, row });

            self.last_spawn_time = now; // 重置计时器
            // 可以动态调整 spawn_interval 以增加难度
            // self.spawn_interval = Duration::from_secs(rng.gen_range(5..12));
        }

        spawns
    }
}
