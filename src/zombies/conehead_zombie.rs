//! # 路障僵尸模块 (`conehead_zombie`)
//!
//! 实现了游戏中的一种特殊僵尸——路障僵尸。
//! 路障僵尸比普通僵尸拥有更高的初始生命值，因为它头戴路障作为额外护甲。
//! 当其生命值降低到一定程度（相当于失去路障后），它会转变为一个普通僵尸。

use crate::zombies::zombie_trait::ZombieTrait;
use ggez::graphics::Image; // 移除了未使用的 DrawParam
use crate::core::resources::Resources;
use crate::zombies::normal_zombie::NormalZombie;
// use crate::zombies::zombie_factory::ZombieType; // ZombieType 未在此文件中直接使用

/// 路障僵尸的初始生命值。
const INITIAL_HEALTH: i32 = 370; // 路障僵尸有更多的生命值
/// 普通僵尸的生命值阈值。
/// 当路障僵尸的生命值低于此值时，其行为和外观将转变为普通僵尸。
const NORMAL_ZOMBIE_HEALTH: i32 = 200; 
/// 路障僵尸的移动速度（像素/毫秒）。
const SPEED: f32 = 0.017; 
/// 路障僵尸的攻击伤害值。
const ATTACK_DAMAGE: i32 = 300; 
/// 路障僵尸的攻击间隔（毫秒）。
const ATTACK_INTERVAL: u64 = 1000; // 攻击间隔为1秒

/// 路障僵尸的结构体实现。
///
/// `ConeheadZombie` 内部包含一个 `NormalZombie` 实例，用于在路障被破坏后
/// 模拟其行为转变为普通僵尸。它还跟踪自身是否已转变以及当前的生命值，
/// 以便正确处理伤害和动画。
pub struct ConeheadZombie {
    /// 标记路障僵尸是否已经失去了路障并转变为普通僵尸形态。
    transformed_to_normal: bool,
    /// 一个内部的 `NormalZombie` 实例。
    /// 当 `transformed_to_normal` 为 `true` 时，路障僵尸的许多行为会委托给此实例。
    normal_zombie: NormalZombie,
    /// 路障僵尸当前的生命值。
    /// 用于判断何时应该失去路障并转变形态。
    current_health: i32,
}

impl ConeheadZombie {
    /// 创建一个新的 `ConeheadZombie` 实例。
    ///
    /// 初始化时，僵尸拥有路障，生命值为 `INITIAL_HEALTH`。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `ConeheadZombie`。
    pub fn new() -> Self {
        ConeheadZombie {
            transformed_to_normal: false,
            normal_zombie: NormalZombie::new(),
            current_health: INITIAL_HEALTH,
        }
    }
}

impl ZombieTrait for ConeheadZombie {
    /// 获取路障僵尸的初始生命值。
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    /// 获取路障僵尸的移动速度。
    fn get_speed(&self) -> f32 {
        SPEED
    }
    
    /// 获取路障僵尸的攻击伤害。
    fn get_attack_damage(&self) -> i32 {
        ATTACK_DAMAGE
    }
    
    /// 获取路障僵尸的攻击间隔。
    fn get_attack_interval(&self) -> u64 {
        ATTACK_INTERVAL
    }
    
    /// 获取路障僵尸行走动画的总帧数。
    ///
    /// 如果已转变为普通僵尸，则返回普通僵尸的行走帧数。
    fn get_walk_frame_count(&self) -> usize {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的行走帧数
            self.normal_zombie.get_walk_frame_count()
        } else {
            // 路障僵尸行走动画有21帧
            21
        }
    }
    
    /// 获取路障僵尸攻击动画的总帧数。
    ///
    /// 如果已转变为普通僵尸，则返回普通僵尸的攻击帧数。
    fn get_attack_frame_count(&self) -> usize {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的攻击帧数
            self.normal_zombie.get_attack_frame_count()
        } else {
            // 路障僵尸攻击动画有11帧
            11
        }
    }
    
    /// 获取路障僵尸当前行走动画帧对应的图像。
    ///
    /// 如果已转变为普通僵尸，则返回普通僵尸的行走图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回对应帧的图像引用。
    fn get_walk_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的行走图片
            self.normal_zombie.get_walk_image(resources, frame)
        } else {
            // 使用路障僵尸特定的行走图片
            let cone_walk_images = &resources.cone_zombie_walk_images;
            let frame_count = cone_walk_images.len();
            if frame_count > 0 {
                &cone_walk_images[frame % frame_count]
            } else {
                // 如果没有特定图片，回退到普通僵尸图片
                self.normal_zombie.get_walk_image(resources, frame)
            }
        }
    }
    
    /// 获取路障僵尸当前攻击动画帧对应的图像。
    ///
    /// 如果已转变为普通僵尸，则返回普通僵尸的攻击图像。
    ///
    /// # Arguments
    ///
    /// * `resources` - 游戏资源实例的引用。
    /// * `frame` - 当前动画帧的索引。
    ///
    /// # Returns
    ///
    /// 返回对应帧的图像引用。
    fn get_attack_image<'a>(&self, resources: &'a Resources, frame: usize) -> &'a Image {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的攻击图片
            self.normal_zombie.get_attack_image(resources, frame)
        } else {
            // 使用路障僵尸特定的攻击图片
            let cone_attack_images = &resources.cone_zombie_attack_images;
            let frame_count = cone_attack_images.len();
            if frame_count > 0 {
                &cone_attack_images[frame % frame_count]
            } else {
                // 如果没有特定图片，回退到普通僵尸图片
                self.normal_zombie.get_attack_image(resources, frame)
            }
        }
    }
    
    /// 指示路障僵尸是否具有特殊能力（即路障未掉落）。
    ///
    /// # Returns
    ///
    /// 如果路障未掉落 (`transformed_to_normal` 为 `false`)，返回 `true`。
    fn has_special_ability(&self) -> bool {
        !self.transformed_to_normal
    }
    
    /// 处理路障僵尸受到伤害时的特殊逻辑。
    ///
    /// 当路障僵尸受到伤害时，会更新其 `current_health`。
    /// 如果 `current_health` 低于 `NORMAL_ZOMBIE_HEALTH` 且尚未转变，
    /// 则将 `transformed_to_normal` 设置为 `true`，模拟路障掉落。
    ///
    /// # Arguments
    ///
    /// * `damage` - 对僵尸造成的伤害值。
    ///
    /// # Returns
    ///
    /// 如果此次伤害导致了路障掉落（即状态转变），则返回 `true`。
    /// 否则返回 `false`。
    fn handle_damage(&mut self, damage: i32) -> bool {
        // 如果已经变成普通僵尸，没有特殊处理
        if self.transformed_to_normal {
            return false;
        }
        
        // 更新当前生命值跟踪
        self.current_health -= damage;
        
        // 如果生命值低于普通僵尸的生命值，触发路障掉落转变为普通僵尸
        if self.current_health <= NORMAL_ZOMBIE_HEALTH {
            println!("路障僵尸的路障被打掉了，变成了普通僵尸！");
            self.transformed_to_normal = true;
            return true; // 表示有特殊处理
        }
        
        false // 没有触发特殊处理
    }
    
    /// 实现transform_health方法
    fn transform_health(&self) -> Option<i32> {
        if self.transformed_to_normal {
            // 如果已经转变为普通僵尸，返回普通僵尸的健康值
            Some(NORMAL_ZOMBIE_HEALTH)
        } else {
            None
        }
    }
}