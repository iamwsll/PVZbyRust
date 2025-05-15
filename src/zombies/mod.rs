//! # 僵尸模块 (`zombies`)
//!
//! 负责定义和管理游戏中所有类型的僵尸。
//!
//! 该模块包含：
//! - 各种具体僵尸类型的实现（例如 `normal_zombie`、`conehead_zombie`）。
//! - 一个通用的 `Zombie` 结构体，用于在游戏中表示一个僵尸实例，并处理其通用逻辑（如移动、动画、受伤、攻击）。
//! - `ZombieTrait`，一个定义所有僵尸类型必须实现的共享行为的特征。
//! - `ZombieFactory`，用于根据 `ZombieType` 枚举创建具体僵尸实例的工厂。
//!
//! 僵尸从屏幕右侧生成，并向左移动，试图攻击玩家的植物。
//! 它们具有生命值、移动速度、攻击力和攻击间隔等属性。
//! 僵尸还具有行走、攻击和死亡等动画。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Rect}; // DrawParam 在 Zombie::draw 中使用
use crate::core::resources::Resources;
use crate::ui::grid::{GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_START_X};

// 声明子模块
pub mod normal_zombie;
pub mod zombie_trait;
pub mod zombie_factory;
pub mod conehead_zombie;
// 未来可以添加
// pub mod buckethead_zombie;

// 从工厂模块中重新导出僵尸类型枚举和工厂本身
pub use zombie_factory::{ZombieType, ZombieFactory};

/// 代表一个游戏中的僵尸实例。
///
/// `Zombie` 结构体封装了一个具体僵尸实现（通过 `Box<dyn ZombieTrait>`），
/// 并管理其在游戏世界中的状态，如位置、生命值、动画、攻击行为等。
pub struct Zombie {
    /// 僵尸的具体类型（例如，普通僵尸、路障僵尸）。
    zombie_type: ZombieType,
    /// 僵尸所在的行（网格的y索引）。
    pub row: usize,
    /// 僵尸在屏幕上的X坐标。
    pub x: f32,
    /// 僵尸当前的生命值。
    health: i32,
    /// 僵尸的移动速度（像素/毫秒）。
    speed: f32,
    /// 当前动画帧的索引。
    animation_frame: usize,
    /// 动画计时器，用于控制动画帧的切换速率。
    animation_timer: u64,
    /// 标记僵尸当前是否正在攻击。
    attacking: bool, 
    /// 标记僵尸是否正在播放死亡动画。
    pub is_dying: bool, 
    /// 标记僵尸的死亡动画是否已播放完成。
    pub death_animation_complete: bool, 
    
    /// 僵尸的攻击伤害值。
    attack_damage: i32,     
    /// 僵尸的攻击间隔（毫秒）。
    attack_interval: u64,   
    /// 攻击计时器，用于控制攻击频率。
    attack_timer: u64,      
    /// 如果正在攻击，则为目标植物在植物列表中的索引。
    attack_target: Option<usize>, 
    
    /// 标记僵尸的头部是否正在掉落（作为死亡动画的一部分）。
    head_falling: bool, 
    /// 头部掉落动画的当前帧索引。
    head_animation_frame: usize, 
    /// 头部掉落动画的计时器。
    head_animation_timer: u64, 
    /// 掉落头部的X坐标。
    head_x: f32, 
    /// 掉落头部的Y坐标。
    head_y: f32, 
    
    /// 对具体僵尸行为实现的动态分发。
    /// 这是一个实现了 `ZombieTrait` 的对象，包含了特定僵尸类型的逻辑。
    zombie_impl: Box<dyn zombie_trait::ZombieTrait>,
}

impl Zombie {
    /// 创建一个新的 `Zombie` 实例。
    ///
    /// # Arguments
    ///
    /// * `zombie_type` - 要创建的僵尸的类型 (`ZombieType`)。
    /// * `row` - 僵尸生成的行号（网格y索引）。
    ///
    /// # Returns
    ///
    /// 返回一个初始化后的 `Zombie` 实例。
    /// 僵尸会从屏幕右侧外部开始移动。
    pub fn new(zombie_type: ZombieType, row: usize) -> Self {
        // 使用工厂创建具体僵尸实现
        let zombie_impl = ZombieFactory::create_zombie(zombie_type);
        
        // 获取僵尸基本属性
        let health = zombie_impl.get_initial_health();
        let speed = zombie_impl.get_speed();
        let attack_damage = zombie_impl.get_attack_damage();
        let attack_interval = zombie_impl.get_attack_interval();

        Zombie {
            zombie_type,
            row,
            x: 950.0, // 从屏幕右侧开始，确保完全在屏幕外生成
            health,
            speed,
            animation_frame: 0,
            animation_timer: 0,
            attacking: false,
            is_dying: false,
            death_animation_complete: false,
            
            // 初始化攻击相关字段
            attack_damage,
            attack_interval,
            attack_timer: 0,
            attack_target: None,
            
            // 初始化头部动画相关字段
            head_falling: false,
            head_animation_frame: 0,
            head_animation_timer: 0,
            head_x: 0.0,
            head_y: 0.0,
            
            // 具体僵尸实现
            zombie_impl,
        }
    }

    /// 更新僵尸的状态。
    ///
    /// 此方法处理僵尸的动画更新（行走、攻击、死亡、头部掉落），
    /// 以及在非攻击状态下的移动。
    /// 它还会调用具体僵尸实现的 `update_special` 方法以处理特定逻辑。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上一帧以来经过的时间（毫秒）。
    pub fn update(&mut self, dt: u64) {
        // 如果僵尸正在死亡，处理死亡动画
        if self.is_dying {
            self.animation_timer += dt;
            if self.animation_timer > 200 { // 每200ms更新一帧死亡动画
                // 死亡动画总共有10帧
                if self.animation_frame < 9 { // 0-9共10帧
                    self.animation_frame += 1;
                    
                    // 在死亡动画第2帧时触发头部掉落
                    if self.animation_frame == 2 && !self.head_falling {
                        self.head_falling = true;
                        // 设置头部初始位置，相对于僵尸的位置
                        self.head_x = self.x + 40.0; // TODO：根据僵尸图像调整头部位置的偏移量
                        let y = GRID_START_Y + (self.row as f32) * GRID_CELL_HEIGHT - GRID_CELL_HEIGHT/4.0;
                        self.head_y = y + 20.0; //TODO： 根据僵尸图像调整头部位置的偏移量
                    }
                } else {
                    // 动画播放完成
                    self.death_animation_complete = true;
                }
                self.animation_timer = 0;
            }
            
            // 处理头部掉落动画
            if self.head_falling {
                self.head_animation_timer += dt;
                if self.head_animation_timer > 150 { // 头部动画比主体动画稍快
                    // 头部动画总共有12帧
                    if self.head_animation_frame < 11 { // 0-11共12帧
                        self.head_animation_frame += 1;
                    }
                    self.head_animation_timer = 0;
                }
            }
            
            return; // 正在死亡时不执行其他更新逻辑
        }

        // 行走动画更新
        self.animation_timer += dt;
        if self.animation_timer > 200 {
            let frame_count = if self.attacking {
                self.zombie_impl.get_attack_frame_count()
            } else {
                self.zombie_impl.get_walk_frame_count()
            };
            
            self.animation_frame = (self.animation_frame + 1) % frame_count;
            self.animation_timer = 0;
        }

        // 移动逻辑
        if !self.attacking {
            self.x -= self.speed * dt as f32;
        }

        // 调用特定僵尸的更新逻辑
        self.zombie_impl.update_special(dt);
    }

    /// 在屏幕上绘制僵尸。
    ///
    /// 根据僵尸的当前状态（行走、攻击、死亡）选择并绘制相应的动画帧。
    /// 如果僵尸正在死亡且头部掉落动画已触发，则还会绘制掉落的头部。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的 `Context` 引用。
    /// * `resources` - 游戏资源 (`Resources`) 的引用，用于获取图像。
    ///
    /// # Returns
    ///
    /// 如果绘制成功，返回 `GameResult<()>`。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算僵尸在屏幕上的 Y 坐标
        let y = GRID_START_Y + (self.row as f32) * GRID_CELL_HEIGHT - GRID_CELL_HEIGHT/4.0;

        // 根据僵尸状态选择图像
        let image = if self.is_dying {
            // 使用死亡动画
            self.zombie_impl.get_die_image(resources, self.animation_frame)
        } else if self.attacking {
            // 使用攻击动画
            self.zombie_impl.get_attack_image(resources, self.animation_frame)
        } else {
            // 使用行走动画
            self.zombie_impl.get_walk_image(resources, self.animation_frame)
        };

        // 获取僵尸特定的绘制参数
        let mut draw_params = self.zombie_impl.get_draw_params();
        draw_params = draw_params.dest([self.x, y]);

        // 绘制僵尸主体
        graphics::draw(ctx, image, draw_params)?;
        
        // 如果头部正在掉落，绘制头部动画
        if self.head_falling && !self.death_animation_complete {
            let frame_count = resources.zombie_head_images.len();
            if frame_count > 0 && self.head_animation_frame < frame_count {
                let head_image = &resources.zombie_head_images[self.head_animation_frame];
                graphics::draw(
                    ctx,
                    head_image,
                    DrawParam::default()
                        .dest([self.head_x, self.head_y])
                        .scale([0.7, 0.7]), // 头部图像可以稍微小一点
                )?;
            }
        }
        
        Ok(())
    }

    /// 获取僵尸的碰撞矩形。
    ///
    /// 用于检测僵尸与植物或豌豆的碰撞。
    /// 碰撞区域通常会根据僵尸图像进行调整，以获得更准确的碰撞效果。
    ///
    /// # Returns
    ///
    /// 返回一个 `Rect` 结构，表示僵尸的碰撞边界框。
    pub fn get_rect(&self) -> Rect {
        let y = GRID_START_Y + (self.row as f32) * GRID_CELL_HEIGHT - GRID_CELL_HEIGHT/4.0;
        
        // 僵尸的碰撞区域应该比显示的图像小一些，以使游戏更加公平 .TODO: 根据实际图像大小调整
        let width = 20.0;
        let height = 100.0;
        
        Rect::new(self.x + 40.0, y + 20.0, width, height)
    }
    
    /// 处理僵尸受到的伤害。
    ///
    /// 首先，它会调用具体僵尸实现的 `handle_damage` 方法，
    /// 这可能导致特殊效果（如路障僵尸失去路障并转变形态）。
    /// 如果特殊效果处理了伤害或导致生命值重置，则按此逻辑执行。
    /// 否则，直接扣减僵尸的生命值。
    /// 如果生命值降至0或以下，僵尸将进入死亡状态并开始播放死亡动画。
    ///
    /// # Arguments
    ///
    /// * `damage` - 对僵尸造成的伤害值。
    ///
    /// # Returns
    ///
    /// 如果僵尸因此次伤害而死亡，则返回 `true`，否则返回 `false`。
    pub fn take_damage(&mut self, damage: i32) -> bool {
        // 先检查是否有特殊伤害处理逻辑（如路障掉落等）
        let damage_handled = self.zombie_impl.handle_damage(damage);
        
        if damage_handled {
            // 如果有特殊处理，检查是否需要更新健康值
            if let Some(new_health) = self.zombie_impl.transform_health() {
                // 如果需要转变健康值（例如变成普通僵尸）
                self.health = new_health;
                println!("僵尸转变形态，健康值重置为: {}", self.health);
            }
        } else {
            // 只有当特殊处理没有应用伤害时，才减去伤害值
            self.health -= damage;
            println!("僵尸受到{}点伤害，剩余生命值: {}", damage, self.health);
        }
        
        // 检查是否死亡
        if self.health <= 0 {
            // 设置死亡动画状态
            self.is_dying = true;
            // 重置动画帧以便从头播放死亡动画
            self.animation_frame = 0;
            self.animation_timer = 0;
            return true;
        }
        
        false
    }
    
    /// 执行僵尸对植物的攻击。
    ///
    /// 此方法由外部逻辑（例如 `EntityManager`）在检测到僵尸与植物碰撞时调用。
    /// 它使用内部计时器 `attack_timer` 来控制攻击频率。
    /// 当达到攻击间隔时，对目标植物的生命值 (`plant_health`) 造成伤害。
    ///
    /// # Arguments
    ///
    /// * `plant_health` - 一个可变引用，指向目标植物的当前生命值。
    /// * `dt` - 自上一帧以来经过的时间（毫秒），用于更新攻击计时器。
    pub fn attack_plant(&mut self, plant_health: &mut i32, dt: u64) {
        // 增加攻击计时器
        self.attack_timer += dt;
        
        // 如果达到攻击间隔时间，执行攻击
        if self.attack_timer >= self.attack_interval {
            // 对植物造成伤害
            *plant_health -= self.attack_damage;
            println!("僵尸啃咬了植物，造成{}点伤害，植物剩余生命值: {}", self.attack_damage, *plant_health);
            
            // 重置攻击计时器
            self.attack_timer = 0;
        }
    }
    
    /// 检查在僵尸前方（同一行）是否有植物。
    ///
    /// 用于判断僵尸是否应该停止移动并开始攻击。
    ///
    /// # Arguments
    ///
    /// * `plant_grid_x` - 植物所在的网格列索引。
    /// * `plant_grid_y` - 植物所在的网格行索引。
    ///
    /// # Returns
    ///
    /// 如果植物在僵尸前方且在同一行，并且它们的碰撞区域接触，则返回 `true`。
    /// 否则返回 `false`。
    pub fn has_plant_in_front(&self, plant_grid_x: usize, plant_grid_y: usize) -> bool {
        // 如果植物不在同一行，则不可能碰撞
        if self.row != plant_grid_y {
            return false;
        }
        
        // 获取植物的屏幕坐标 (左边缘),这个位置是微调出来的
        let plant_left_edge = GRID_START_X + (plant_grid_x as f32) * GRID_CELL_WIDTH - GRID_CELL_WIDTH;
        
        // 获取僵尸的前部坐标 (右边缘)
        let zombie_right_edge = self.x + 20.0; // TODO:根据僵尸大小调整
        
        // 如果僵尸的右边缘达到或超过植物的左边缘，则视为碰撞
        // 通常加一个小的偏移量使碰撞更接近图形效果
        zombie_right_edge >= plant_left_edge && zombie_right_edge <= plant_left_edge + GRID_CELL_WIDTH
    }
    
    /// 设置僵尸的攻击状态。
    ///
    /// 当僵尸开始或停止攻击时调用此方法。
    /// 它会更新 `attacking` 状态和 `attack_target`，
    /// 并在开始攻击时重置动画和攻击计时器。
    ///
    /// # Arguments
    ///
    /// * `is_attacking` - 布尔值，`true` 表示开始攻击，`false` 表示停止攻击。
    /// * `target_index` - `Option<usize>`，如果开始攻击，则为目标植物的索引；否则为 `None`。
    pub fn set_attacking(&mut self, is_attacking: bool, target_index: Option<usize>) {
        if self.attacking != is_attacking {
            self.attacking = is_attacking;
            self.attack_target = target_index;
            
            // 重置攻击相关的动画
            if is_attacking {
                self.animation_frame = 0;
                self.animation_timer = 0;
                self.attack_timer = 0;
            }
        }
    }
    
    /// 获取僵尸的类型。
    ///
    /// # Returns
    ///
    /// 返回僵尸的 `ZombieType` 枚举成员。
    pub fn get_zombie_type(&self) -> ZombieType {
        self.zombie_type
    }
}
