use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, DrawMode, Color, DrawParam};

pub struct Grid {
    occupied: [[bool; 9]; 5],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            occupied: [[false; 9]; 5],
        }
    }

    pub fn get_grid_position(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x < 40.0 || x > 740.0 || y < 80.0 || y > 480.0 {
            return None;
        }
        
        let grid_x = ((x - 40.0) / 80.0) as usize;
        let grid_y = ((y - 80.0) / 80.0) as usize;
        
        if grid_x < 9 && grid_y < 5 {
            Some((grid_x, grid_y))
        } else {
            None
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.occupied[y][x]
    }

    pub fn occupy(&mut self, x: usize, y: usize) {
        self.occupied[y][x] = true;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        // 可选的：绘制网格线以便调试
        for i in 0..=9 {
            let x = 40.0 + i as f32 * 80.0;
            let line = Mesh::new_line(
                ctx,
                &[[x, 80.0], [x, 480.0]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 0.2),
            )?;
            graphics::draw(ctx, &line, DrawParam::default())?;
        }

        for i in 0..=5 {
            let y = 80.0 + i as f32 * 80.0;
            let line = Mesh::new_line(
                ctx,
                &[[40.0, y], [740.0, y]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 0.2),
            )?;
            graphics::draw(ctx, &line, DrawParam::default())?;
        }
        
        Ok(())
    }
}
