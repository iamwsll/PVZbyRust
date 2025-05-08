use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;
use ggez::graphics::Rect;

// 豌豆类型枚举
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PeaType {
    Normal,  // 普通豌豆
    // Snow,    // 寒冰豌豆
}

// 豌豆结构体
pub struct Pea {
    pub x: f32,         // x坐标
    pub y: f32,         // y坐标
    pub row: usize,     // 所在行
    pub speed: f32,     // 移动速度
    pub damage: i32,    // 伤害值
    pub pea_type: PeaType, // 豌豆类型
    pub active: bool,   // 是否有效
}

impl Pea {
    // 创建一个新豌豆
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

    // 更新豌豆位置
    pub fn update(&mut self, dt: u64) {
        self.x += self.speed * dt as f32;
        
        // 如果豌豆飞出屏幕，将其设置为非活动状态
        if self.x > 900.0 {
            self.active = false;
        }
    }

    // 绘制豌豆
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

    // 获取豌豆的碰撞矩形
    pub fn get_rect(&self) -> Rect {
        // 豌豆碰撞范围，根据实际图片大小调整 TODO：调整
        Rect::new(self.x, self.y, 20.0, 20.0)
    }

}