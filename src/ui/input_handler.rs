//! # 输入处理模块 (`input_handler`)
//!
//! 负责处理游戏中的用户输入，主要是鼠标点击事件。
//! 它管理着阳光的收集、商店中植物卡片的选择、以及在网格上放置植物的逻辑。

use crate::ui::grid::{Grid, GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT, GRID_CELL_WIDTH, GRID_WIDTH, GRID_HEIGHT};
use crate::plants::{Plant, PlantType};
use crate::ui::shop::{Shop, SHOP_START_Y, CARD_HEIGHT};
use crate::entities::sun::Sun;
use ggez::input::mouse::MouseButton;

/// `InputHandler` 结构体。
///
/// 这是一个单元结构体，目前仅包含静态方法用于处理输入事件。
/// 它不持有任何状态，而是直接操作传入的游戏状态变量。
pub struct InputHandler;

impl InputHandler {
    /// 处理鼠标按下事件。
    ///
    /// 根据鼠标点击的位置和按键，执行相应的游戏操作，例如：
    /// - 点击阳光：收集阳光并增加阳光计数。
    /// - 点击商店卡片：如果阳光充足，则选择相应的植物类型以备放置。
    /// - 在网格上点击（已选择植物时）：如果单元格未被占据且阳光充足，则放置植物。
    /// - 右键点击：取消当前选择的植物。
    ///
    /// # Arguments
    ///
    /// * `button` - 按下的鼠标按键 (`ggez::input::mouse::MouseButton`)。
    /// * `x` - 鼠标点击的屏幕X坐标。
    /// * `y` - 鼠标点击的屏幕Y坐标。
    /// * `shop` - 可变的 `Shop` 引用，用于处理商店交互和更新选择状态。
    /// * `suns` - 可变的阳光实体列表 (`Vec<Sun>`)，用于检查点击和移除被收集的阳光。
    /// * `grid` - 可变的 `Grid` 引用，用于检查网格位置和标记占据状态。
    /// * `plants` - 可变的植物实体列表 (`Vec<Plant>`)，用于添加新放置的植物。
    /// * `selected_plant` - 可变的 `Option<PlantType>`，表示当前从商店选择的植物类型。
    /// * `sun_count` - 可变的阳光计数器 (`i32`)。
    /// * `game_over` - 一个布尔值，指示游戏是否已结束。如果为 `true`，则不处理输入。
    ///
    /// # Returns
    ///
    /// 如果输入事件导致了游戏状态的改变（例如收集了阳光、选择了植物、放置了植物），
    /// 则返回 `true`，否则返回 `false`。
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
                   if Self::place_plant(x, y, grid, plants, selected_plant, sun_count, shop) {
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

    /// 处理在网格上放置植物的逻辑。
    ///
    /// 当玩家在选择了植物类型后点击网格时，此方法被调用。
    /// 它会检查目标网格单元格是否有效、是否已被占据，以及玩家是否有足够的阳光。
    /// 如果所有条件满足，则创建一个新的植物实例，将其添加到游戏世界，
    /// 更新阳光数量，标记网格单元格为已占据，并清除当前的植物选择状态。
    ///
    /// # Arguments
    ///
    /// * `x` - 鼠标点击的屏幕X坐标。
    /// * `y` - 鼠标点击的屏幕Y坐标。
    /// * `grid` - 可变的 `Grid` 引用。
    /// * `plants` - 可变的植物实体列表 (`Vec<Plant>`)。
    /// * `selected_plant` - 可变的 `Option<PlantType>`，包含待放置的植物类型。
    /// * `sun_count` - 可变的阳光计数器 (`i32`)。
    ///
    /// # Returns
    ///
    /// 如果植物成功放置，则返回 `true`。否则（例如，位置无效、已被占据、阳光不足），
    /// 返回 `false`。
    fn place_plant(
        x: f32, 
        y: f32, 
        grid: &mut Grid,
        plants: &mut Vec<Plant>,
        selected_plant: &mut Option<PlantType>,
        sun_count: &mut i32,
        shop: &mut Shop
    ) -> bool {
        if let Some(plant_type) = selected_plant {
            if let Some((grid_x, grid_y)) = grid.get_grid_position(x, y) {
                // 检查是否已有植物
                if !grid.is_occupied(grid_x, grid_y) && *sun_count >= plant_type.cost() {
                    let plant = Plant::new(*plant_type, grid_x, grid_y);
                    plants.push(plant);
                    *sun_count -= plant_type.cost();
                    grid.occupy(grid_x, grid_y);

                    // 在植物成功放置后，才触发卡片冷却
                    shop.trigger_card_cooldown(*plant_type);

                    // 放置植物后取消选择状态
                    *selected_plant = None;

                    return true;
                }
            }
        }
        false
    }
}