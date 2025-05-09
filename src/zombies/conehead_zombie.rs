// src/zombies/conehead_zombie.rs
use crate::zombies::zombie_trait::ZombieTrait;
use ggez::graphics::{Image, DrawParam};
use crate::core::resources::Resources;
use crate::zombies::normal_zombie::NormalZombie;
use crate::zombies::zombie_factory::ZombieType;

// 路障僵尸的特定属性
const INITIAL_HEALTH: i32 = 370; // 路障僵尸有更多的生命值
const NORMAL_ZOMBIE_HEALTH: i32 = 200; // 普通僵尸的生命值，当路障僵尸的生命值低于这个值时，将变成普通僵尸
const SPEED: f32 = 0.02; // 每毫秒移动的像素
const ATTACK_DAMAGE: i32 = 50; // 每次攻击造成的伤害
const ATTACK_INTERVAL: u64 = 1000; // 攻击间隔为1秒

/// 路障僵尸的具体实现
pub struct ConeheadZombie {
    // 记录是否已经转化为普通僵尸
    transformed_to_normal: bool,
    // 内部保存一个普通僵尸实例，当路障掉落后使用
    normal_zombie: NormalZombie,
    // 跟踪当前生命值，用于判断是否需要变形
    current_health: i32,
}

impl ConeheadZombie {
    pub fn new() -> Self {
        ConeheadZombie {
            transformed_to_normal: false,
            normal_zombie: NormalZombie::new(),
            current_health: INITIAL_HEALTH,
        }
    }
}

impl ZombieTrait for ConeheadZombie {
    fn get_initial_health(&self) -> i32 {
        INITIAL_HEALTH
    }
    
    fn get_speed(&self) -> f32 {
        SPEED
    }
    
    fn get_attack_damage(&self) -> i32 {
        ATTACK_DAMAGE
    }
    
    fn get_attack_interval(&self) -> u64 {
        ATTACK_INTERVAL
    }
    
    // 覆盖行走动画帧数
    fn get_walk_frame_count(&self) -> usize {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的行走帧数
            self.normal_zombie.get_walk_frame_count()
        } else {
            // 路障僵尸行走动画有21帧
            21
        }
    }
    
    // 覆盖攻击动画帧数
    fn get_attack_frame_count(&self) -> usize {
        if self.transformed_to_normal {
            // 变成普通僵尸后使用普通僵尸的攻击帧数
            self.normal_zombie.get_attack_frame_count()
        } else {
            // 路障僵尸攻击动画有11帧
            11
        }
    }
    
    // 覆盖行走动画图片
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
    
    // 覆盖攻击动画图片
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
    
    // 路障僵尸有特殊能力
    fn has_special_ability(&self) -> bool {
        !self.transformed_to_normal
    }
    
    // 处理路障僵尸受到伤害时的特殊逻辑
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
    
    // 实现transform_health方法
    fn transform_health(&self) -> Option<i32> {
        if self.transformed_to_normal {
            // 如果已经转变为普通僵尸，返回普通僵尸的健康值
            Some(NORMAL_ZOMBIE_HEALTH)
        } else {
            None
        }
    }
}