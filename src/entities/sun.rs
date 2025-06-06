//! # 阳光实体模块
//!
//! 定义了游戏中阳光的行为和属性，阳光是用于购买植物的主要资源。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::core::resources::Resources;

/// 阳光的生成类型枚举。
///
/// 用于区分阳光是自然从天上掉落还是由向日葵等植物产生。
pub enum SunType {
    /// 自然生成的阳光，通常从屏幕顶端掉落到随机位置。
    NaturalGeneration,
    /// 由向日葵产生的阳光，通常出现在向日葵附近。
    SunflowerGeneration,
}

/// 阳光结构体，代表游戏中的可收集资源。
///
/// 包含了阳光的位置、目标y坐标（用于自然掉落动画）、下落速度、
/// 生命周期计时器、动画帧相关信息以及其生成类型。
pub struct Sun {
    /// 阳光当前的x轴坐标。
    x: f32,
    /// 阳光当前的y轴坐标。
    y: f32,
    /// 自然掉落阳光的目标y轴坐标。
    target_y: f32,
    /// 阳光（特指自然掉落类型）的下落速度。
    speed: f32,
    /// 阳光已存在的时间，可用于实现例如超时消失等逻辑 (当前未使用)。
    lifetime: u64,
    /// 当前显示的阳光动画帧的索引。
    animation_frame: usize,
    /// 用于控制阳光动画帧切换的计时器。
    animation_timer: u64,
    /// 阳光的生成类型（自然掉落或向日葵产生）。
    sun_type: SunType,
    /// 向日葵阳光的初始y坐标，用于跳跃动画
    initial_y: f32,
    /// 向日葵阳光的跳跃高度
    jump_height: f32,
    /// 向日葵阳光的跳跃阶段 (0-100表示百分比)
    jump_phase: f32,
    /// 向日葵阳光的跳跃速度
    jump_speed: f32,
}

impl Sun {
    /// 创建一个新的阳光实例。
    ///
    /// # Arguments
    ///
    /// * `x` - 阳光的初始x坐标。
    /// * `y` - 阳光的初始y坐标。
    /// * `gen_sun_type` - 阳光的生成类型 (`SunType`)。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Sun` 实例。
    pub fn new(x: f32, y: f32, gen_sun_type:SunType) -> Self {
        let target_y = rand::random::<f32>() * 400.0 + 200.0;
        
        Sun {
            x,
            y,
            target_y,
            speed: 0.06,
            lifetime: 0,
            animation_frame: 0,
            animation_timer: 0,
            sun_type: gen_sun_type,
            initial_y: y,
            jump_height: 20.0, // 跳跃高度，稍微增加使动画更明显
            jump_phase: 0.0,   // 初始为0
            jump_speed: 0.2,   // 跳跃速度，加快以使动画更快完成
        }
    }

    /// 更新阳光的状态。
    ///
    /// 根据阳光的类型执行不同的更新逻辑：
    /// - `NaturalGeneration`: 使阳光向下移动直到达到 `target_y`。
    /// - `SunflowerGeneration`: 当前保持在原地（未来可添加浮动等动画）。
    /// 同时更新阳光的动画帧。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒）。
    pub fn update(&mut self, dt: u64) {
        // 根据阳光类型执行不同的更新逻辑
        match self.sun_type {
            SunType::NaturalGeneration => {
                // 自然产生的阳光下落
                if self.y < self.target_y {
                    self.y += self.speed * dt as f32;
                    if self.y > self.target_y {
                        self.y = self.target_y;
                    }
                }
            }
            SunType::SunflowerGeneration => {
                // 向日葵产生的阳光只执行一次向上跳跃动画
                if self.jump_phase < 100.0 {
                    // 只有在初始阶段才更新jump_phase，达到100后停止更新
                    self.jump_phase += self.jump_speed * dt as f32;
                    
                    // 创建一个类似抛物线的跳跃动画
                    // 当jump_phase为0时，阳光在初始位置
                    // 当jump_phase接近50时，阳光达到最高点
                    // 当jump_phase接近100时，阳光回到原始位置
                    let progress = self.jump_phase.min(100.0) / 100.0;
                    
                    // 使用抛物线函数 y = -4(x-0.5)^2 + 1 来生成跳跃轨迹
                    // 这会在x=0.5时达到最大值1，在x=0和x=1时为0
                    let parabola = -4.0 * (progress - 0.5) * (progress - 0.5) + 1.0;
                    
                    // 计算当前的跳跃高度
                    let current_jump_height = parabola * self.jump_height;
                    
                    // 更新阳光的y坐标，实现向上跳跃效果
                    self.y = self.initial_y - current_jump_height;
                }
                // 当jump_phase >= 100.0时，不再更新位置，阳光保持在原位

            }
        }


        self.lifetime += dt;
        
        // 动画帧更新逻辑保
        self.animation_timer += dt;
        if self.animation_timer > 50 {  // 缩短切换时间使动画更流畅
            self.animation_frame = (self.animation_frame + 1) % 22;  // 在22帧之间循环
            self.animation_timer = 0;
        }
    }

    /// 绘制单个阳光到屏幕上。
    ///
    /// 使用 `sun_images` 中的当前动画帧进行绘制。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `resources` - 游戏资源，用于获取阳光的动画帧图像。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        graphics::draw(
            ctx,
            &resources.sun_images[self.animation_frame],  // 使用当前动画帧
            DrawParam::default()
                .dest([self.x, self.y])
                .scale([0.6, 0.6]),
        )
    }

    /// 检测给定的点坐标是否在阳光的可点击范围内。
    ///
    /// # Arguments
    ///
    /// * `x` - 要检测的点的x坐标。
    /// * `y` - 要检测的点的y坐标。
    ///
    /// # Returns
    ///
    /// 如果点在阳光的点击半径内，则返回 `true`，否则返回 `false`。
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let radius = 40.0; // 阳光的点击半径
        let dx = self.x + radius - x;
        let dy = self.y + radius - y;
        dx * dx + dy * dy <= radius * radius
    }
}
