//! # 铲子模块 (`shovel`)
//!
//! 该模块实现了游戏中铲子的功能，允许玩家铲除已种植的植物。
//! 铲子的图像显示在商店旁边的铲子框内，玩家可以拖动铲子铲除网格中的植物。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Rect};
use crate::core::resources::Resources;
use crate::ui::grid::Grid;
use crate::plants::Plant;

/// 铲子在屏幕上的默认X坐标。
pub const SHOVEL_DEFAULT_X: f32 = 700.0;
/// 铲子在屏幕上的默认Y坐标。
pub const SHOVEL_DEFAULT_Y: f32 = 8.0;
/// 铲子的宽度。
pub const SHOVEL_WIDTH: f32 = 70.0;
/// 铲子的高度。
pub const SHOVEL_HEIGHT: f32 = 70.0;
/// 铲子框的宽度。
pub const SHOVEL_BANK_WIDTH: f32 = 80.0;
/// 铲子框的高度。
pub const SHOVEL_BANK_HEIGHT: f32 = 80.0;

/// 铲子结构体，用于管理铲子的状态和功能。
pub struct Shovel {
    /// 铲子在屏幕上的位置 (x, y)。
    pub position: (f32, f32),
    /// 铲子是否被拖动中。
    pub is_dragging: bool,
    /// 铲子的矩形区域，用于碰撞检测（点击）。
    pub rect: Rect,
    /// 铲子框的矩形区域。
    pub bank_rect: Rect,
}

impl Shovel {
    /// 创建一个新的铲子实例。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Shovel` 实例，位于默认位置。
    pub fn new() -> Self {
        Shovel {
            position: (SHOVEL_DEFAULT_X, SHOVEL_DEFAULT_Y),
            is_dragging: false,
            rect: Rect::new(SHOVEL_DEFAULT_X, SHOVEL_DEFAULT_Y, SHOVEL_WIDTH, SHOVEL_HEIGHT),
            bank_rect: Rect::new(SHOVEL_DEFAULT_X - 5.0, SHOVEL_DEFAULT_Y - 5.0, SHOVEL_BANK_WIDTH, SHOVEL_BANK_HEIGHT),
        }
    }

    /// 更新铲子位置（当被拖动时）。
    ///
    /// # Arguments
    ///
    /// * `x` - 鼠标的X坐标。
    /// * `y` - 鼠标的Y坐标。
    pub fn update_position(&mut self, x: f32, y: f32) {
        if self.is_dragging {
            self.position = (x - SHOVEL_WIDTH / 2.0, y - SHOVEL_HEIGHT / 2.0);
            self.rect.x = self.position.0;
            self.rect.y = self.position.1;
        }
    }

    /// 检查给定的坐标是否在铲子上。
    ///
    /// # Arguments
    ///
    /// * `x` - 鼠标的X坐标。
    /// * `y` - 鼠标的Y坐标。
    ///
    /// # Returns
    ///
    /// 如果点击在铲子上，返回 `true`；否则返回 `false`。
    pub fn is_clicked(&self, x: f32, y: f32) -> bool {
        self.rect.contains([x, y])
    }

    /// 尝试铲除指定位置的植物。
    ///
    /// # Arguments
    ///
    /// * `x` - 鼠标的X坐标。
    /// * `y` - 鼠标的Y坐标。
    /// * `grid` - 可变的 `Grid` 引用。
    /// * `plants` - 可变的植物实体列表 (`Vec<Plant>`)。
    ///
    /// # Returns
    ///
    /// 如果成功铲除了植物，返回 `true`；否则返回 `false`。
    pub fn dig(&self, x: f32, y: f32, grid: &mut Grid, plants: &mut Vec<Plant>) -> bool {
        if let Some((grid_x, grid_y)) = grid.get_grid_position(x, y) {
            if grid.is_occupied(grid_x, grid_y) {
                // 找到并移除对应位置的植物
                let plant_index = plants.iter().position(|plant| 
                    plant.grid_x == grid_x && plant.grid_y == grid_y
                );
                
                if let Some(index) = plant_index {
                    plants.remove(index);
                    
                    // 将该位置标记为未占用
                    grid.unoccupy(grid_x, grid_y);
                    
                    return true;
                }
            }
        }
        false
    }



    /// 绘制铲子和铲子框。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的 `Context` 引用。
    /// * `resources` - 游戏资源的引用，包含铲子和铲子框的图像。
    ///
    /// # Returns
    ///
    /// 如果绘制成功，则返回 `GameResult<()>`。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 绘制铲子框
        let bank_param = DrawParam::default()
            .dest([self.bank_rect.x, self.bank_rect.y])
            .scale([self.bank_rect.w / resources.shovel_bank_image.width() as f32,
                    self.bank_rect.h / resources.shovel_bank_image.height() as f32]);
        graphics::draw(ctx, &resources.shovel_bank_image, bank_param)?;
        
        // 绘制铲子
        let shovel_param = DrawParam::default()
            .dest([self.rect.x, self.rect.y])
            .scale([self.rect.w / resources.shovel_image.width() as f32,
                    self.rect.h / resources.shovel_image.height() as f32]);
        graphics::draw(ctx, &resources.shovel_image, shovel_param)?;
        
        Ok(())
    }

    /// 将铲子重置到默认位置。
    pub fn reset(&mut self) {
        self.position = (SHOVEL_DEFAULT_X, SHOVEL_DEFAULT_Y);
        self.rect.x = self.position.0;
        self.rect.y = self.position.1;
        self.is_dragging = false;
    }
}
