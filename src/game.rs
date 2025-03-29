use crate::grid::Grid;
use crate::plant::{Plant, PlantType};
use crate::resources::Resources;
use crate::sun::Sun;
use crate::zombie::{Zombie, ZombieType};
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
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let resources = Resources::new(ctx)?;
        let grid = Grid::new();

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
        //以每秒DESIRED_FPS帧的速度更新游戏状态
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if !self.game_over {
                // // 更新僵尸
                // for zombie in &mut self.zombies {
                //     zombie.update(dt);
                // }

                // // 更新植物
                // for plant in &mut self.plants {
                //     plant.update(dt);
                // }

                // 更新阳光
                for sun in &mut self.suns {
                    sun.update(dt);
                }

                // // 生成僵尸逻辑
                // self.spawn_timer += ggez::timer::delta(ctx);
                // if self.spawn_timer.as_secs() >= 20 {
                //     self.spawn_zombie();
                //     self.spawn_timer = Duration::from_secs(0);
                // }

                // 随机生成阳光
                if rand::random::<u32>() % 300 == 0 {
                    self.spawn_sun();
                }

                // 碰撞检测和游戏逻辑
                // ...
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // 绘制背景
        let bg = &self.resources.background;
        graphics::draw(ctx, bg, DrawParam::default())?;

        // 绘制网格（可选）
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

        // 绘制UI
        // ...

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

            // 放置植物
            if x >= 40.0 && x <= 740.0 && y >= 80.0 && y <= 480.0 {
                self.place_plant(x, y);
            }
            // 选择植物逻辑
            // ...
        }
    }
}
