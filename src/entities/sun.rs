use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::core::resources::Resources;

pub enum SunType {
    NaturalGeneration,
    SunflowerGeneration,
}
pub struct Sun {
    x: f32,
    y: f32,
    target_y: f32,
    speed: f32,
    lifetime: u64,
    animation_frame: usize,    // 当前动画帧索引
    animation_timer: u64,      // 动画计时器
    sun_type: SunType,
}

impl Sun {
    pub fn new(x: f32, y: f32, gen_sun_type:SunType) -> Self {
        let target_y = rand::random::<f32>() * 400.0 + 200.0;
        
        Sun {
            x,
            y,
            target_y,
            speed: 0.1,
            lifetime: 0,
            animation_frame: 0,
            animation_timer: 0,
            sun_type: gen_sun_type,
        }
    }

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
                // 向日葵产生的阳光暂时保持在原地不动
                // 未来可以添加其他行为，例如轻微浮动或短暂的上升动画
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

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        graphics::draw(
            ctx,
            &resources.sun_images[self.animation_frame],  // 使用当前动画帧
            DrawParam::default()
                .dest([self.x, self.y])
                .scale([0.6, 0.6]),
        )
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let radius = 40.0; // 阳光的点击半径
        let dx = self.x + radius - x;
        let dy = self.y + radius - y;
        dx * dx + dy * dy <= radius * radius
    }
}
