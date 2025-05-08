use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Rect};
use crate::resources::Resources;

// 声明子模块
pub mod normal;
// pub mod conehead; // 未来可以添加
// pub mod buckethead; // 未来可以添加

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZombieType {
    Normal,
    // Conehead,
    // Buckethead,
}

pub struct Zombie {
    zombie_type: ZombieType,
    pub row: usize,
    pub x: f32, // Make x public for game logic access
    health: i32,
    speed: f32,
    animation_frame: usize,
    animation_timer: u64,
    attacking: bool, // 僵尸是否在攻击 (未来用于与植物交互)
    pub is_dying: bool, // 僵尸是否正在死亡（播放死亡动画）
    pub death_animation_complete: bool, // 死亡动画是否播放完成
    
    // 头掉落相关字段
    head_falling: bool, // 僵尸头是否正在掉落
    head_animation_frame: usize, // 头部动画当前帧
    head_animation_timer: u64, // 头部动画计时器
    head_x: f32, // 头部X坐标
    head_y: f32, // 头部Y坐标
}

impl Zombie {
    pub fn new(zombie_type: ZombieType, row: usize) -> Self {
        let (health, speed) = match zombie_type {
            ZombieType::Normal => (normal::INITIAL_HEALTH, normal::SPEED),
            // ZombieType::Conehead => (conehead::INITIAL_HEALTH, conehead::SPEED),
            // ZombieType::Buckethead => (buckethead::INITIAL_HEALTH, buckethead::SPEED),
        };

        Zombie {
            zombie_type,
            row,
            x: 950.0, // 从屏幕更右侧开始，确保完全在屏幕外生成
            health,
            speed,
            animation_frame: 0,
            animation_timer: 0,
            attacking: false,
            is_dying: false,
            death_animation_complete: false,
            
            // 初始化头部动画相关字段
            head_falling: false,
            head_animation_frame: 0,
            head_animation_timer: 0,
            head_x: 0.0,
            head_y: 0.0,
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

        // 行走动画更新 (通用逻辑)
        self.animation_timer += dt;
        if self.animation_timer > 200 { // 动画切换速度，每200ms绘制动画的一帧。
            // 根据僵尸类型和状态选择不同的动画帧范围
            let frame_count = 22; // 默认行走动画有22帧
            self.animation_frame = (self.animation_frame + 1) % frame_count;
            self.animation_timer = 0;
        }

        // 移动逻辑 (通用逻辑)
        if !self.attacking {
            self.x -= self.speed * dt as f32;
        }

        // TODO: 调用特定僵尸的更新逻辑 (如果需要)
        // match self.zombie_type {
        //     ZombieType::Normal => normal::update(),
        //     ...
        // }

        // TODO: 添加攻击逻辑检查 (当僵尸遇到植物时，设置 attacking = true)
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算僵尸在屏幕上的 Y 坐标 (通用逻辑)
        let y = crate::grid::GRID_START_Y + (self.row as f32) * crate::grid::GRID_CELL_HEIGHT - crate::grid::GRID_CELL_HEIGHT/4.0;

        // 根据僵尸状态选择图像
        let image = if self.is_dying {
            // 使用死亡动画
            let frame_count = resources.zombie_die_images.len();
            if frame_count > 0 && self.animation_frame < frame_count {
                &resources.zombie_die_images[self.animation_frame]
            } else {
                println!("No death animation frame available at index {}", self.animation_frame);
                // 如果没有对应的死亡帧，使用最后一帧
                if !resources.zombie_die_images.is_empty() {
                    &resources.zombie_die_images[resources.zombie_die_images.len() - 1]
                } else {
                    // 如果没有死亡动画，回退到行走动画的第一帧
                    &resources.zombies_walk1_images[0]
                }
            }
        } else {
            // 非死亡状态，使用普通行走/攻击动画
            match self.zombie_type {
                ZombieType::Normal => {
                    // TODO: 根据 attacking 状态选择行走或攻击动画
                    let frame_count = resources.zombies_walk1_images.len();
                    if frame_count > 0 {
                        &resources.zombies_walk1_images[self.animation_frame % frame_count]
                    } else {
                        println!("No images available for Normal Zombie");
                        &resources.zombies_walk1_images[0]
                    }
                }
                // 处理其他僵尸类型...
            }
        };

        // 绘制僵尸主体
        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([self.x, y])
                .scale([0.8, 0.8]), // 僵尸图像缩放比例
        )?;
        
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
    
}
