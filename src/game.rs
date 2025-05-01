use crate::grid::{Grid, GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT,GRID_CELL_WIDTH, GRID_WIDTH, GRID_HEIGHT};
use crate::plants::{Plant, PlantType};
use crate::resources::Resources;
use crate::sun::{Sun, SunType};
use crate::zombies::{Zombie, ZombieType}; // Use the zombies module
use crate::shop::Shop;
use crate::level_controller::LevelController; // 游戏进度/难度控制器
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

// 游戏状态
pub struct GameState {
    resources: Resources,
    grid: Grid,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    suns: Vec<Sun>,
    sun_count: i32,
    selected_plant: Option<PlantType>,//选中的植物类型。用来放置植物有关的内容
    game_over: bool,
    shop: Shop,
    level_controller: LevelController, // 添加关卡控制器
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let resources = Resources::new(ctx)?;
        let grid = Grid::new();
        let shop = Shop::new();
        let level_controller = LevelController::new();

        Ok(GameState {
            resources,
            grid,
            plants: Vec::new(),
            zombies: Vec::new(),
            suns: Vec::new(),
            sun_count: 50,
            selected_plant: None,
            game_over: false,
            shop,
            level_controller, // 添加到 GameState
        })
    }

    //产生僵尸的抽象
    fn spawn_zombie(&mut self, zombie_type: ZombieType, row: usize) {
        let zombie = Zombie::new(zombie_type, row);
        self.zombies.push(zombie);
    }
    // 产生阳光的抽象
    fn spawn_sun(&mut self) {
        let x = rand::random::<f32>() * 700.0 + 50.0;
        let sun = Sun::new(x, 0.0, SunType::NaturalGeneration);
        self.suns.push(sun);
    }
    // 处理植物放置逻辑
    fn place_plant(&mut self, x: f32, y: f32) -> bool {
        if let Some(plant_type) = &self.selected_plant {
            if let Some((grid_x, grid_y)) = self.grid.get_grid_position(x, y) {
                // 检查是否已有植物
                if !self.grid.is_occupied(grid_x, grid_y) && self.sun_count >= plant_type.cost() {
                    let plant = Plant::new(*plant_type, grid_x, grid_y);
                    self.plants.push(plant);
                    self.sun_count -= plant_type.cost();
                    self.grid.occupy(grid_x, grid_y);

                    // 放置植物后取消选择状态
                    self.selected_plant = None;
                    self.shop.selected_plant = None;

                    return true;
                }
            }
        }
        false
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //游戏帧率，时间间隔
        const DESIRED_FPS: u32 = 60;
        let dt = ggez::timer::delta(ctx).as_millis() as u64;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if !self.game_over {
                // 更新阳光数组的持续时间
                for sun in &mut self.suns {
                    sun.update(dt);
                }

                // 更新植物
                let mut new_suns = Vec::new();
                for plant in &mut self.plants {
                    plant.update(dt, &mut new_suns);
                }
                // 把阳光类植物添加进来
                self.suns.append(&mut new_suns); 

                // 更新僵尸
                for zombie in &mut self.zombies {
                    zombie.update(dt);
                }

                // 检查游戏是否结束。放在更新僵尸之后。
                // TODO：更加严谨的做法
                if self.game_over { continue; } 


                // // 移除死亡的植物和僵尸 (示例，需要更完善的碰撞和生命值处理)
                // self.plants.retain(|plant| plant.is_alive());
                // self.zombies.retain(|zombie| zombie.is_alive());

                // TODO: 添加碰撞检测和处理逻辑 (植物 vs 僵尸, 子弹 vs 僵尸)


                // 随机生成自然阳光
                if rand::random::<u32>() % 500 == 0 { //TODO：太过粗糙，当前实际上需要修改
                    self.spawn_sun();
                }

                // 更新关卡控制器并生成僵尸
                let zombie_spawns = self.level_controller.update(dt);
                for spawn_info in zombie_spawns {
                    self.spawn_zombie(spawn_info.zombie_type, spawn_info.row);
                }

                // // 定时生成僵尸 - 逻辑已移至 LevelController
                // let now = Instant::now();
                // if now.duration_since(self.last_zombie_spawn_time) >= self.zombie_spawn_interval {
                //     self.spawn_zombie(); // Old call removed
                //     self.last_zombie_spawn_time = now; // Reset the timer
                // }


                // 更新商店
                self.shop.update(self.sun_count);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // 绘制背景
        let bg = &self.resources.background;
        graphics::draw(ctx, bg, DrawParam::default())?;

        // 绘制商店面板
        let shop_img = &self.resources.shop_image; // Use the correct variable name
        graphics::draw(ctx, shop_img, DrawParam::default().dest([250.0, 0.0]))?;

        //绘制网格（调试用）
        self.grid.draw(ctx)?; // Uncomment to see grid lines

        // 绘制植物
        for plant in &self.plants {
            plant.draw(ctx, &self.resources)?;
        }

        // 绘制僵尸
        for zombie in &self.zombies {
            zombie.draw(ctx, &self.resources)?;
        }

        // 绘制阳光
        for sun in &self.suns {
            sun.draw(ctx, &self.resources)?;
        }

        // 绘制商店卡片和选中植物预览
        self.shop.draw(ctx, &self.resources)?;

        // 绘制阳光数量
        let sun_text = graphics::Text::new(
            graphics::TextFragment::new(format!("{}", self.sun_count))
                .color(Color::BLACK)
                .scale(25.0) // Make text larger
        );
        // Adjust position to be inside the sun counter area on the shop image
        graphics::draw(
            ctx,
            &sun_text,
            DrawParam::default().dest([285.0, 65.0]) // Adjusted position
        )?;

        // 如果游戏结束，显示 Game Over 消息
        if self.game_over {
            let game_over_text = graphics::Text::new(
                graphics::TextFragment::new("GAME OVER")
                    .color(Color::RED)
                    .scale(100.0),
            );
            let text_width = game_over_text.width(ctx);
            let text_height = game_over_text.height(ctx);
            let screen_size = graphics::drawable_size(ctx);
            graphics::draw(
                ctx,
                &game_over_text,
                DrawParam::default().dest([
                    screen_size.0 / 2.0 - text_width / 2.0,
                    screen_size.1 / 2.0 - text_height / 2.0,
                ]),
            )?;
        }


        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.game_over {
            return;
        }

        if button == MouseButton::Left {
            // 检查是否点击了阳光
            self.suns.retain(|sun| {
                if sun.contains_point(x, y) {
                    self.sun_count += 25;
                    false // Remove the sun
                } else {
                    true // Keep the sun
                }
            });
            // if self.sun_count > initial_sun_count { return; } // If sun was clicked, don't process other clicks

            // 处理商店卡片点击 (优先于放置植物)
            // Check if click is within shop bounds first
             if y < crate::shop::SHOP_START_Y + crate::shop::CARD_HEIGHT + 20.0 { // Approximate shop area check
                 if let Some(plant_type) = self.shop.handle_click(x, y, self.sun_count) {
                     self.selected_plant = Some(plant_type);
                     // Deduct sun cost ONLY when placing, not selecting
                     return; // Stop processing if a card was clicked
                 }
             }


            // 处理植物放置逻辑 (仅当有植物被选中且点击在网格内时)
            if self.selected_plant.is_some() {
                 if x >= GRID_START_X && x <= GRID_START_X + GRID_CELL_WIDTH * GRID_WIDTH as f32 &&
                    y >= GRID_START_Y && y <= GRID_START_Y + GRID_CELL_HEIGHT * GRID_HEIGHT as f32 {
                    if self.place_plant(x, y) {
                       // Sun cost is handled inside place_plant
                    } else {
                        // If placement failed (e.g., occupied cell, not enough sun), deselect plant
                        self.selected_plant = None;
                        self.shop.selected_plant = None; // Also clear shop selection state
                    }
                } else {
                     // Clicked outside grid while plant selected, deselect
                     self.selected_plant = None;
                     self.shop.selected_plant = None;
                }
            }
        } else if button == MouseButton::Right {
             // Right-click cancels plant selection
             self.selected_plant = None;
             self.shop.selected_plant = None;
        }
    }
}
