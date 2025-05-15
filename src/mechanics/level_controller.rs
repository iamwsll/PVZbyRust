//! # 关卡控制器模块 (`level_controller`)
//!
//! 本模块负责管理游戏的关卡流程，特别是僵尸的生成逻辑。
//! 它决定了何时、何种类型以及在哪一行生成僵尸，从而控制游戏的难度和节奏。

use crate::zombies::{ZombieType, Zombie};
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
/// 管理僵尸的生成时机、类型和波次。它实现了完整的关卡流程控制，
/// 包括初始等待时间、波次管理、难度递增和胜利条件判定。
pub struct LevelController {
    /// 上一次生成僵尸的时间点。
    last_spawn_time: Instant,
    /// 生成僵尸之间的时间间隔。
    spawn_interval: Duration,
    /// 关卡开始的时间点。
    start_time: Instant,
    /// 当前波次，从0开始。
    current_wave: usize,
    /// 本关卡的总波数。
    total_waves: usize,
    /// 当前波次中已生成的僵尸数量。
    zombies_spawned_in_wave: usize,
    /// 当前波次需要生成的僵尸总数。
    zombies_per_wave: usize,
    /// 标记最后一波是否已经提示。
    final_wave_announced: bool,
    /// 标记所有僵尸是否已生成完毕。
    all_zombies_spawned: bool,
    /// 是否正在等待开始第一波。
    waiting_for_first_wave: bool,
}

impl LevelController {
    /// 创建一个新的 `LevelController` 实例。
    ///
    /// 初始化时，设置关卡开始时间为当前时间，并配置波次和生成规律。
    /// 第一波僵尸将在关卡开始后12秒出现。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `LevelController` 实例。
    pub fn new() -> Self {
        LevelController {
            last_spawn_time: Instant::now(),
            spawn_interval: Duration::from_secs(2), // 同一波内僵尸生成间隔
            start_time: Instant::now(),
            current_wave: 0,
            total_waves: 7, // 总共7波僵尸
            zombies_spawned_in_wave: 0,
            zombies_per_wave: 1, // 第一波只有1个僵尸
            final_wave_announced: false,
            all_zombies_spawned: false,
            waiting_for_first_wave: true,
        }
    }
    
    /// 获取当前是否已经宣布最后一波来袭。
    ///
    /// 当最后一波僵尸出现时，此方法将返回`true`，可用于在UI上显示"最后一波来袭"的通知。
    /// 注意：此方法会设置内部标记，确保只返回一次`true`。
    ///
    /// # Returns
    ///
    /// 如果最后一波已经开始且尚未宣布，则返回`true`；否则返回`false`。
    pub fn is_final_wave_announced(&mut self) -> bool {
        if self.current_wave == self.total_waves - 1 && !self.final_wave_announced {
            self.final_wave_announced = true;
            return true;
        }
        false
    }
    
    /// 检查关卡是否胜利完成。
    ///
    /// 当所有波次的僵尸都已生成且场上没有活着的僵尸时，表示关卡胜利。
    ///
    /// # Arguments
    ///
    /// * `zombies` - 当前场上所有僵尸的引用切片。
    ///
    /// # Returns
    ///
    /// 如果满足胜利条件，则返回`true`；否则返回`false`。
    pub fn is_level_completed(&self, zombies: &[Zombie]) -> bool {
        if self.all_zombies_spawned && zombies.is_empty() {
            return true;
        }
        false
    }
    
    /// 检查当前场上是否还有僵尸存活。
    ///
    /// # Arguments
    ///
    /// * `zombies` - 当前场上所有僵尸的引用切片。
    ///
    /// # Returns
    ///
    /// 如果场上没有活着的僵尸，则返回`true`；否则返回`false`。
    pub fn is_wave_cleared(&self, zombies: &[Zombie]) -> bool {
        zombies.is_empty()
    }

    /// 更新关卡控制器的状态，并决定是否需要生成新的僵尸。
    ///
    /// 实现了完整的波次管理逻辑：
    /// 1. 游戏开始后12秒开始第一波
    /// 2. 每波内生成指定数量的僵尸
    /// 3. 波次之间有休息时间
    /// 4. 难度随波次递增
    /// 5. 最后一波时会标记"最后一波来袭"
    ///
    /// # Arguments
    ///
    /// * `_dt` - 自上次更新以来的时间增量（毫秒）。
    ///
    /// # Returns
    ///
    /// 返回一个 `Vec<ZombieSpawnInfo>`，其中包含所有在本次更新中需要生成的僵尸的信息。
    /// 如果不需要生成僵尸，则返回空向量。
    pub fn update(&mut self, _dt: u64, zombies: &[Zombie]) -> Vec<ZombieSpawnInfo> {
        let mut spawns = Vec::new();
        let now = Instant::now();
        
        // 如果已经生成了所有僵尸，直接返回
        if self.all_zombies_spawned {
            return spawns;
        }
        
        // 等待游戏开始后12秒再生成第一波
        if self.waiting_for_first_wave {
            if now.duration_since(self.start_time) < Duration::from_secs(12) {
                return spawns; // 继续等待
            }
            // 12秒后开始第一波
            self.waiting_for_first_wave = false;
            self.last_spawn_time = now; // 立即开始生成第一只僵尸
        }
        
        // 检查是否需要生成新僵尸
        if now.duration_since(self.last_spawn_time) >= self.spawn_interval {
            // 首先检查当前波次是否已经生成了足够数量的僵尸
            if self.zombies_spawned_in_wave < self.zombies_per_wave {
                // 在波次内生成一个僵尸
                let mut rng = rand::thread_rng();
                let row = rng.gen_range(0..GRID_HEIGHT);
                
                // 根据波次决定僵尸类型，只有第四波开始才会出现路障僵尸
                let zombie_type = if self.current_wave >= 3 && rng.gen_range(0..10) < (self.current_wave - 2) {
                    ZombieType::Conehead
                } else {
                    ZombieType::Normal
                };
                
                spawns.push(ZombieSpawnInfo { zombie_type, row });
                
                self.zombies_spawned_in_wave += 1;
                self.last_spawn_time = now; // 重置计时器
                
                println!("生成僵尸: 波次 {}, 僵尸 {}/{}", 
                         self.current_wave + 1, 
                         self.zombies_spawned_in_wave, 
                         self.zombies_per_wave);
            } 
            // 如果当前波次中的僵尸已经全部生成完毕，并且场上没有存活的僵尸，考虑进入下一波
            else if self.is_wave_cleared(zombies) {
                // 波次间隔为15秒
                if now.duration_since(self.last_spawn_time) >= Duration::from_secs(15) {
                    // 进入下一波
                    self.current_wave += 1;
                    
                    // 判断是否所有波次已完成
                    if self.current_wave >= self.total_waves {
                        self.all_zombies_spawned = true;
                        return spawns;
                    }
                    
                    // 重置当前波次的僵尸计数
                    self.zombies_spawned_in_wave = 0;
                    
                    // 根据波次设置僵尸数量
                    if self.current_wave == 1 || self.current_wave == 2 {
                        // 第二、三波各2只僵尸
                        self.zombies_per_wave = 2;
                    } else if self.current_wave >= 3 {
                        // 后续波次每波递增1只
                        self.zombies_per_wave = self.current_wave;
                    }
                    
                    // 最后一波前减少生成间隔，增加难度
                    if self.current_wave == self.total_waves - 1 {
                        self.spawn_interval = Duration::from_secs(1);
                    }
                    
                    println!("波次 {} 已开始，僵尸数量: {}", self.current_wave + 1, self.zombies_per_wave);
                    self.last_spawn_time = now; // 重置计时器，准备生成新一波的僵尸
                }
            }
        }
        
        spawns
    }
}
