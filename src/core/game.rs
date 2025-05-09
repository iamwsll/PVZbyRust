use crate::ui::grid::Grid;
use crate::plants::PlantType;
use crate::core::resources::Resources;
use crate::entities::sun::Sun;
use crate::zombies::Zombie;
use crate::entities::pea::Pea;
use crate::ui::shop::Shop;
use crate::plants::Plant;
use crate::mechanics::collision::CollisionManager;
use crate::mechanics::entity_manager::EntityManager;
use crate::ui::input_handler::InputHandler;
use crate::core::renderer::Renderer;

use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

/// 游戏状态结构体，包含游戏运行所需的所有状态
pub struct GameState {
    // 资源
    resources: Resources,
    
    // 游戏实体
    grid: Grid,
    plants: Vec<Plant>,
    zombies: Vec<Zombie>,
    suns: Vec<Sun>,
    peas: Vec<Pea>,
    
    // 游戏状态
    sun_count: i32,
    selected_plant: Option<PlantType>,
    game_over: bool,
    
    // 功能模块
    shop: Shop,
    entity_manager: EntityManager,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let resources = Resources::new(ctx)?;
        let grid = Grid::new();
        let shop = Shop::new();
        let entity_manager = EntityManager::new();

        Ok(GameState {
            resources,
            grid,
            plants: Vec::new(),
            zombies: Vec::new(),
            suns: Vec::new(),
            peas: Vec::new(),
            sun_count: 50,
            selected_plant: None,
            game_over: false,
            shop,
            entity_manager,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 设置游戏帧率
        const DESIRED_FPS: u32 = 60;
        let dt = ggez::timer::delta(ctx).as_millis() as u64;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.game_over {
                continue;
            }
            
            // 更新阳光
            for sun in &mut self.suns {
                sun.update(dt);
            }

            // 更新植物并收集新产生的阳光
            let mut new_suns = Vec::new();
            for plant in &mut self.plants {
                plant.update(dt, &mut new_suns, &mut self.peas);
            }
            self.suns.append(&mut new_suns);

            // 更新僵尸
            for zombie in &mut self.zombies {
                zombie.update(dt);
            }

            // 更新豌豆
            for pea in &mut self.peas {
                pea.update(dt);
            }

            // 处理碰撞逻辑
            CollisionManager::handle_pea_zombie_collision(&mut self.peas, &mut self.zombies);
            CollisionManager::handle_zombie_plant_interaction(&mut self.zombies, &mut self.plants, ctx);

            // 检查游戏是否结束
            for zombie in &self.zombies {
                if zombie.x <= 0.0 {
                    self.game_over = true;
                    break;
                }
            }

            if self.game_over { continue; }

            // 随机生成自然阳光
            if EntityManager::should_spawn_natural_sun() {
                let new_sun = self.entity_manager.spawn_natural_sun();
                self.suns.push(new_sun);
            }

            // 通过关卡控制器更新并生成僵尸
            let zombie_spawns = self.entity_manager.update(dt);
            for spawn_info in zombie_spawns {
                let zombie = self.entity_manager.spawn_zombie(spawn_info.zombie_type, spawn_info.row);
                self.zombies.push(zombie);
            }

            // 更新商店
            self.shop.update(self.sun_count);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Renderer::draw_game(
            ctx,
            &self.resources,
            &self.grid,
            &self.plants,
            &self.peas,
            &self.zombies,
            &self.suns,
            &self.shop,
            self.sun_count,
            self.game_over
        )
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        InputHandler::handle_mouse_down(
            button, 
            x, 
            y,
            &mut self.shop,
            &mut self.suns,
            &mut self.grid,
            &mut self.plants,
            &mut self.selected_plant,
            &mut self.sun_count,
            self.game_over
        );
    }
}
