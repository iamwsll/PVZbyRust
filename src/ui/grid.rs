//! # 游戏网格模块 (`grid`)
//!
//! 定义了游戏区域的网格布局，包括其尺寸、单元格大小以及相关操作。
//! 网格用于确定植物的放置位置、僵尸的移动路径以及其他游戏元素的空间关系。

use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, Color, DrawParam};

// 定义网格位置常量
// -------->x 
// |
// |
// |
// v 
// y

/// 网格在屏幕上的起始X坐标（左边距）。
pub const GRID_START_X: f32 = 254.0;  // 左边距
/// 网格在屏幕上的起始Y坐标（上边距）。
pub const GRID_START_Y: f32 = 75.0;  // 上边距
/// 网格中每个单元格的高度。
pub const GRID_CELL_HEIGHT: f32 = 100.0; // 单元格高度
/// 网格中每个单元格的宽度。
pub const GRID_CELL_WIDTH: f32 = 80.0; // 单元格宽度
/// 网格的宽度（列数）。
pub const GRID_WIDTH: usize = 9;      // 网格宽度（列数）
/// 网格的高度（行数）。
pub const GRID_HEIGHT: usize = 5;     // 网格高度（行数）

/// 代表游戏区域的网格结构。
///
/// `Grid` 结构体维护一个二维数组 `occupied` 来跟踪哪些单元格已被植物占据。
/// 它提供了将屏幕坐标转换为网格坐标、检查单元格占用状态以及在屏幕上绘制网格线的方法。
pub struct Grid {
    /// 一个二维布尔数组，标记网格单元格是否被占据。
    /// `occupied[y][x]` 为 `true` 表示 (x, y) 位置的单元格被占据。
    occupied: [[bool; GRID_WIDTH]; GRID_HEIGHT], // 使用常量定义数组大小
}

impl Grid {
    /// 创建一个新的 `Grid` 实例。
    ///
    /// 初始化时，所有网格单元格都标记为未被占据。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Grid` 实例。
    pub fn new() -> Self {
        Grid {
            occupied: [[false; GRID_WIDTH]; GRID_HEIGHT], // 使用常量初始化
        }
    }

    /// 将屏幕像素坐标转换为网格坐标。
    ///
    /// # Arguments
    ///
    /// * `x` - 屏幕上的X坐标。
    /// * `y` - 屏幕上的Y坐标。
    ///
    /// # Returns
    ///
    /// 如果给定的屏幕坐标在网格范围内，则返回 `Some((grid_x, grid_y))`，
    /// 其中 `grid_x` 是列索引，`grid_y` 是行索引。
    /// 如果坐标在网格范围之外，则返回 `None`。
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

    /// 检查指定的网格单元格是否已被占据。
    ///
    /// # Arguments
    ///
    /// * `x` - 网格的列索引。
    /// * `y` - 网格的行索引。
    ///
    /// # Returns
    ///
    /// 如果单元格 `(x, y)` 被占据，则返回 `true`，否则返回 `false`。
    /// **注意**: 此方法假设 `x` 和 `y` 是有效的网格索引，不进行边界检查。
    /// 调用者应确保索引在 `0..GRID_WIDTH` 和 `0..GRID_HEIGHT` 范围内。
    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.occupied[y][x]
    }

    /// 将指定的网格单元格标记为已占据。
    ///
    /// # Arguments
    ///
    /// * `x` - 网格的列索引。
    /// * `y` - 网格的行索引。
    ///
    /// **注意**: 此方法假设 `x` 和 `y` 是有效的网格索引，不进行边界检查。
    /// 调用者应确保索引在 `0..GRID_WIDTH` 和 `0..GRID_HEIGHT` 范围内。
    pub fn occupy(&mut self, x: usize, y: usize) {
        self.occupied[y][x] = true;
    }
    
    /// 将指定的网格单元格标记为未占据。
    ///
    /// # Arguments
    ///
    /// * `x` - 网格的列索引。
    /// * `y` - 网格的行索引。
    ///
    /// **注意**: 此方法假设 `x` 和 `y` 是有效的网格索引，不进行边界检查。
    /// 调用者应确保索引在 `0..GRID_WIDTH` 和 `0..GRID_HEIGHT` 范围内。
    pub fn unoccupy(&mut self, x: usize, y: usize) {
        self.occupied[y][x] = false;
    }

    /// 在屏幕上绘制网格线。
    ///
    /// 此方法使用ggez的图形功能来渲染网格的垂直和水平线。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的 `Context` 引用，用于绘图操作。
    ///
    /// # Returns
    ///
    /// 如果绘制成功，则返回 `GameResult<()>`。
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
