use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;

pub struct Sun {
    x: f32,
    y: f32,
    target_y: f32,
    speed: f32,
    lifetime: u64,
}

impl Sun {
    pub fn new(x: f32, y: f32) -> Self {
        let target_y = rand::random::<f32>() * 400.0 + 100.0;
        
        Sun {
            x,
            y,
            target_y,
            speed: 0.5,
            lifetime: 0,
        }
    }

    pub fn update(&mut self, dt: u64) {
        if self.y < self.target_y {
            self.y += self.speed * dt as f32;
            if self.y > self.target_y {
                self.y = self.target_y;
            }
        }

        self.lifetime += dt;
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        graphics::draw(
            ctx,
            &resources.sun_image,
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
