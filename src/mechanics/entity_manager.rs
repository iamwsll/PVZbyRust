use crate::zombies::{Zombie, ZombieType};
use crate::entities::sun::{Sun, SunType};
use crate::ui::grid::GRID_HEIGHT;
use crate::mechanics::level_controller::{LevelController, ZombieSpawnInfo};
use rand::Rng;

/// 实体生成管理模块，负责处理游戏实体的生成、管理逻辑
pub struct EntityManager {
    pub level_controller: LevelController,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            level_controller: LevelController::new(),
        }
    }

    /// 更新实体管理器，处理僵尸生成
    pub fn update(&mut self, dt: u64) -> Vec<ZombieSpawnInfo> {
        self.level_controller.update(dt)
    }

    /// 生成僵尸
    pub fn spawn_zombie(&self, zombie_type: ZombieType, row: usize) -> Zombie {
        Zombie::new(zombie_type, row)
    }

    /// 生成自然阳光
    pub fn spawn_natural_sun(&self) -> Sun {
        let x = rand::thread_rng().gen_range(50.0..750.0);
        Sun::new(x, 0.0, SunType::NaturalGeneration)
    }

    /// 检查是否应该生成自然阳光
    pub fn should_spawn_natural_sun() -> bool {
        // 使用随机率决定是否生成阳光
        rand::random::<u32>() % 500 == 0
    }
}