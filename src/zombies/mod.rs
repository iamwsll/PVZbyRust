use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Rect};
use crate::resources::Resources;

// 声明子模块
pub mod normal_zombie;
pub mod zombie_trait;
pub mod zombie_factory;
// 未来可以添加
// pub mod conehead_zombie;
// pub mod buckethead_zombie;

// 从工厂模块中重新导出僵尸类型枚举
pub use zombie_factory::{ZombieType, ZombieFactory};

// 僵尸结构体，持有具体僵尸实现
pub struct Zombie {
    zombie_type: ZombieType,
    pub row: usize,
    pub x: f32, // Make x public for game logic access
    health: i32,
    max_health: i32, // 记录最大生命值，用于特殊效果
    speed: f32,
    animation_frame: usize,
    animation_timer: u64,
    attacking: bool, // 僵尸是否在攻击
    pub is_dying: bool, // 僵尸是否正在死亡（播放死亡动画）
    pub death_animation_complete: bool, // 死亡动画是否播放完成
    
    // 攻击相关字段
    attack_damage: i32,     // 攻击伤害
    attack_interval: u64,   // 攻击间隔（毫秒）
    attack_timer: u64,      // 攻击计时器
    attack_target: Option<usize>, // 攻击目标植物的索引
    
    // 头掉落相关字段
    head_falling: bool, // 僵尸头是否正在掉落
    head_animation_frame: usize, // 头部动画当前帧
    head_animation_timer: u64, // 头部动画计时器
    head_x: f32, // 头部X坐标
    head_y: f32, // 头部Y坐标
    
    // 具体僵尸实现
    zombie_impl: Box<dyn zombie_trait::ZombieTrait>,
}

impl Zombie {
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
            max_health: health, // 初始时最大生命值等于当前生命值
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
                        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0;
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

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算僵尸在屏幕上的 Y 坐标
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0;

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

    // 获取僵尸的碰撞矩形
    pub fn get_rect(&self) -> Rect {
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0;
        
        // 僵尸的碰撞区域应该比显示的图像小一些，以使游戏更加公平 .TODO: 根据实际图像大小调整
        let width = 20.0;
        let height = 100.0;
        
        Rect::new(self.x + 40.0, y + 20.0, width, height)
    }
    
    // 僵尸受到伤害的方法
    pub fn take_damage(&mut self, damage: i32) -> bool {
        // 先检查是否有特殊伤害处理逻辑（如路障掉落等）
        if self.zombie_impl.handle_damage(damage) {
            // 如果有特殊处理，可以在这里添加特效逻辑
            // 但仍然需要减少生命值
        }
        
        self.health -= damage;
        println!("僵尸受到{}点伤害，剩余生命值: {}", damage, self.health);
        
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
    
    // 僵尸攻击植物的方法
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
    
    // 检查是否有植物在前方
    pub fn has_plant_in_front(&self, plant_grid_x: usize, plant_grid_y: usize) -> bool {
        // 如果植物不在同一行，则不可能碰撞
        if self.row != plant_grid_y {
            return false;
        }
        
        // 获取植物的屏幕坐标 (左边缘),这个位置是微调出来的
        let plant_left_edge = crate::grid::GRID_START_X + (plant_grid_x as f32) * crate::grid::GRID_CELL_WIDTH - crate::grid::GRID_CELL_WIDTH;
        
        // 获取僵尸的前部坐标 (右边缘)
        let zombie_right_edge = self.x + 20.0; // TODO:根据僵尸大小调整
        
        // 如果僵尸的右边缘达到或超过植物的左边缘，则视为碰撞
        // 通常加一个小的偏移量使碰撞更接近图形效果
        zombie_right_edge >= plant_left_edge && zombie_right_edge <= plant_left_edge + crate::grid::GRID_CELL_WIDTH
    }
    
    // 设置攻击状态
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
    
    // 获取僵尸类型
    pub fn get_zombie_type(&self) -> ZombieType {
        self.zombie_type
    }
}
