//! # 豌豆实体模块
//!
//! 定义了游戏中豌豆（子弹）的行为和属性。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::core::resources::Resources;
use ggez::graphics::Rect;

/// 豌豆的类型枚举。
///
/// 目前仅定义了普通豌豆，未来可以扩展例如寒冰豌豆等类型。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PeaType {
    /// 普通豌豆，具有标准的飞行速度和伤害。
    Normal,
    // Snow,    // 寒冰豌豆 (暂未实现)
}

/// 豌豆结构体，代表游戏中的飞行子弹。
///
/// 包含了豌豆的位置、所在行、飞行速度、伤害值、类型以及是否激活等状态。
pub struct Pea {
    /// 豌豆当前的x轴坐标。
    pub x: f32,
    /// 豌豆当前的y轴坐标。
    pub y: f32,
    /// 豌豆所在的行索引。
    pub row: usize,
    /// 豌豆的飞行速度。
    pub speed: f32,
    /// 豌豆对僵尸造成的伤害值。
    pub damage: i32,
    /// 豌豆的类型，例如普通或寒冰。
    pub pea_type: PeaType,
    /// 标记豌豆是否处于活动状态。非活动的豌豆将被移除或不再参与碰撞检测。
    pub active: bool,
}

impl Pea {
    /// 创建一个新的豌豆实例。
    ///
    /// 根据指定的豌豆类型，初始化其速度和伤害。
    ///
    /// # Arguments
    ///
    /// * `x` - 豌豆的初始x坐标。
    /// * `y` - 豌豆的初始y坐标。
    /// * `row` - 豌豆所在的行。
    /// * `pea_type` - 要创建的豌豆类型 (`PeaType`)。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Pea` 实例。
    pub fn new(x: f32, y: f32, row: usize, pea_type: PeaType) -> Self {
        let (speed, damage) = match pea_type {
            PeaType::Normal => (0.3, 20),  // 普通豌豆速度和伤害 TODO：进行速度和伤害的调整
            // PeaType::Snow => (0.25, 20),   // 寒冰豌豆速度和伤害
        };

        Pea {
            x,
            y,
            row,
            speed,
            damage,
            pea_type,
            active: true,
        }
    }

    /// 更新豌豆的状态，主要处理其在x轴上的移动。
    ///
    /// 如果豌豆飞出屏幕右侧边界，则将其标记为非活动状态。
    ///
    /// # Arguments
    ///
    /// * `dt` - 自上次更新以来的时间增量（毫秒）。
    pub fn update(&mut self, dt: u64) {
        self.x += self.speed * dt as f32;
        
        // 如果豌豆飞出屏幕，将其设置为非活动状态
        if self.x > 900.0 {
            self.active = false;
        }
    }

    /// 绘制单个豌豆到屏幕上。
    ///
    /// 根据豌豆的类型选择相应的图像进行绘制。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `resources` - 游戏资源，用于获取豌豆图像。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        let image = match self.pea_type {
            PeaType::Normal => &resources.pea_image,
            // PeaType::Snow => &resources.pea_snow_image,
        };

        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([self.x, self.y])
                .scale([0.7, 0.7]),
        )
    }

    /// 获取豌豆的碰撞检测矩形区域。
    ///
    /// # Returns
    ///
    /// 返回一个 `Rect`，代表豌豆在游戏世界中的碰撞边界。
    pub fn get_rect(&self) -> Rect {
        // 豌豆碰撞范围，根据实际图片大小调整 TODO：调整
        Rect::new(self.x, self.y, 20.0, 20.0)
    }

}