use crate::ui::grid::{Grid, GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_WIDTH, GRID_HEIGHT};
use crate::plants::{Plant, PlantType};
use crate::ui::shop::{Shop, SHOP_START_Y, CARD_HEIGHT};
use crate::entities::sun::Sun;
use ggez::input::mouse::MouseButton;

/// 输入处理模块，负责处理用户输入相关的逻辑
pub struct InputHandler;

impl InputHandler {
    /// 处理鼠标按下事件
    pub fn handle_mouse_down(
        button: MouseButton, 
        x: f32, 
        y: f32,
        shop: &mut Shop,
        suns: &mut Vec<Sun>,
        grid: &mut Grid,
        plants: &mut Vec<Plant>,
        selected_plant: &mut Option<PlantType>,
        sun_count: &mut i32,
        game_over: bool
    ) -> bool {
        if game_over {
            return false;
        }

        if button == MouseButton::Left {
            // 检查是否点击了阳光
            let initial_sun_count = *sun_count;
            suns.retain(|sun| {
                if sun.contains_point(x, y) {
                    *sun_count += 25;
                    false // Remove the sun
                } else {
                    true // Keep the sun
                }
            });
            if *sun_count > initial_sun_count { 
                return true; // 如果点击了阳光，不处理其他点击
            }

            // 处理商店卡片点击 (优先于放置植物)
            if y < SHOP_START_Y + CARD_HEIGHT + 20.0 { // 商店区域的大致检查
                if let Some(plant_type) = shop.handle_click(x, y, *sun_count) {
                    *selected_plant = Some(plant_type);
                    return true; // 如果点击了卡片，停止处理
                }
            }

            // 处理植物放置逻辑
            if selected_plant.is_some() {
                if x >= GRID_START_X && x <= GRID_START_X + GRID_CELL_WIDTH * GRID_WIDTH as f32 &&
                   y >= GRID_START_Y && y <= GRID_START_Y + GRID_CELL_HEIGHT * GRID_HEIGHT as f32 {
                   if Self::place_plant(x, y, grid, plants, selected_plant, sun_count) {
                       // 放置成功，清除选择状态
                       shop.selected_plant = None;
                       return true;
                   } else {
                       // 如果放置失败，取消选择
                       *selected_plant = None;
                       shop.selected_plant = None;
                       return false;
                   }
               } else {
                    // 点击在网格外，取消选择
                    *selected_plant = None;
                    shop.selected_plant = None;
                    return false;
               }
            }
        } else if button == MouseButton::Right {
            // 右键取消选择
            *selected_plant = None;
            shop.selected_plant = None;
            return true;
        }
        
        false
    }

    /// 处理植物放置逻辑
    fn place_plant(
        x: f32, 
        y: f32, 
        grid: &mut Grid,
        plants: &mut Vec<Plant>,
        selected_plant: &mut Option<PlantType>,
        sun_count: &mut i32
    ) -> bool {
        if let Some(plant_type) = selected_plant {
            if let Some((grid_x, grid_y)) = grid.get_grid_position(x, y) {
                // 检查是否已有植物
                if !grid.is_occupied(grid_x, grid_y) && *sun_count >= plant_type.cost() {
                    let plant = Plant::new(*plant_type, grid_x, grid_y);
                    plants.push(plant);
                    *sun_count -= plant_type.cost();
                    grid.occupy(grid_x, grid_y);

                    // 放置植物后取消选择状态
                    *selected_plant = None;

                    return true;
                }
            }
        }
        false
    }
}