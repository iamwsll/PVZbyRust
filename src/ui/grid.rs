use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, Color, DrawParam};

// 定义网格位置常量
// -------->x 
// |
// |
// |
// v 
// y
pub const GRID_START_X: f32 = 254.0;  // 左边距
pub const GRID_START_Y: f32 = 75.0;  // 上边距
pub const GRID_CELL_HEIGHT: f32 = 100.0; // 单元格高度
pub const GRID_CELL_WIDTH: f32 = 80.0; // 单元格宽度
pub const GRID_WIDTH: usize = 9;      // 网格宽度（列数）
pub const GRID_HEIGHT: usize = 5;     // 网格高度（行数）

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
        if x < GRID_START_X || x > GRID_START_X + GRID_CELL_WIDTH * GRID_WIDTH as f32 || 
           y < GRID_START_Y || y > GRID_START_Y + GRID_CELL_HEIGHT * GRID_HEIGHT as f32 {
            return None;
        }
        
        let grid_x = ((x - GRID_START_X) / GRID_CELL_WIDTH) as usize;
        let grid_y = ((y - GRID_START_Y) / GRID_CELL_HEIGHT) as usize;
        
        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
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
        // 绘制网格线
        for i in 0..=GRID_WIDTH {
            //绘制的起始x 也就是每一列的左侧位置
            let x = GRID_START_X + i as f32 * GRID_CELL_WIDTH;
            let line = Mesh::new_line(
                ctx,
                //第一个参数是该列的左上角,第二个参数是该列的左下角
                &[[x, GRID_START_Y], [x, GRID_START_Y + GRID_CELL_HEIGHT * GRID_HEIGHT as f32]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 0.2),
            )?;
            graphics::draw(ctx, &line, DrawParam::default())?;
        }

        for i in 0..=GRID_HEIGHT {
            //绘制的起始y 也就是每一行的上侧位置
            let y = GRID_START_Y + i as f32 * GRID_CELL_HEIGHT;
            let line = Mesh::new_line(
                ctx,
                //第一个参数是该行的左上角,第二个参数是该行的右上角
                &[[GRID_START_X, y], [GRID_START_X + GRID_CELL_WIDTH * GRID_WIDTH as f32, y]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 0.2),
            )?;
            graphics::draw(ctx, &line, DrawParam::default())?;
        }
        
        Ok(())
    }
}
