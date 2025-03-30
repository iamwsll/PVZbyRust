use crate::grid::{Grid, GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT,GRID_CELL_WIDTH, GRID_WIDTH, GRID_HEIGHT};
use crate::plant::{Plant, PlantType};
use crate::resources::Resources;
use crate::sun::Sun;
use crate::zombie::{Zombie, ZombieType};
use crate::shop::Shop;  // 添加Shop导入
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use std::time::Duration;

// 游戏状态
pub struct GameState {
    resources: Resources,
    grid: Grid,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    suns: Vec<Sun>,
    sun_count: i32,
    spawn_timer: Duration,
    selected_plant: Option<PlantType>,
    game_over: bool,
    shop: Shop,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let resources = Resources::new(ctx)?;
        let grid = Grid::new();
        let shop = Shop::new();  // 初始化商店

        Ok(GameState {
            resources,
            grid,
            plants: Vec::new(),
            zombies: Vec::new(),
            suns: Vec::new(),
            sun_count: 50, // 初始阳光数
            spawn_timer: Duration::from_secs(0),
            selected_plant: None,
            game_over: false,
            shop,  
        })
    }

    fn spawn_zombie(&mut self) {
        let row = rand::random::<usize>() % 5;
        let zombie = Zombie::new(ZombieType::Normal, row);
        self.zombies.push(zombie);
    }

    fn spawn_sun(&mut self) {
        let x = rand::random::<f32>() * 700.0 + 50.0;
        let sun = Sun::new(x, 0.0);
        self.suns.push(sun);
    }

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
        const DESIRED_FPS: u32 = 60;
        const MILLIS_PER_UPDATE: u64 = 1000 / DESIRED_FPS as u64;
        let dt = ggez::timer::delta(ctx).as_millis() as u64;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if !self.game_over {
                // 更新阳光
                for sun in &mut self.suns {
                    sun.update(dt);
                }

                // 随机生成阳光 这里可以根据需要调整生成频率
                if rand::random::<u32>() % 300 == 0 {
                    self.spawn_sun();
                }

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
        
        // 绘制商店面板到画面最上方
        let shop = &self.resources.shop_image;
        graphics::draw(ctx, shop, DrawParam::default().dest([250.0, 0.0]))?;

        // 绘制网格（调试用）
        self.grid.draw(ctx)?;

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

        // 绘制商店卡片
        self.shop.draw(ctx, &self.resources)?;
        
        // 绘制阳光数量
        let sun_text = graphics::Text::new(
            graphics::TextFragment::new(format!("{}", self.sun_count))
                .color(Color::BLACK)
                .scale(25.0)
        );
        
        graphics::draw(
            ctx,
            &sun_text,
            DrawParam::default().dest([220.0, 30.0])
        )?;

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
                    false
                } else {
                    true
                }
            });

            // 处理植物放置逻辑
            if self.selected_plant.is_some() {
                if x >= GRID_START_X && x <= GRID_START_X + GRID_CELL_WIDTH * GRID_WIDTH as f32 && 
                   y >= GRID_START_Y && y <= GRID_START_Y + GRID_CELL_HEIGHT * GRID_HEIGHT as f32 {
                    self.place_plant(x, y);
                }
            } else {
                // 处理商店卡片点击
                if let Some(plant_type) = self.shop.handle_click(x, y, self.sun_count) {
                    self.selected_plant = Some(plant_type);
                }
            }
        }
    }
}
