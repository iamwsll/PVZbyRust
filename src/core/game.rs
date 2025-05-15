//! # 游戏主逻辑模块
//! 
//! `game` 模块负责实现游戏的主要逻辑，包括游戏状态的初始化、更新、绘制以及事件处理。
//! 它作为游戏的核心控制器，协调各个子系统的工作。

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
use std::time::Instant;

/// 游戏状态结构体，封装了游戏世界中的所有动态数据和状态。
///
/// `GameState` 负责管理游戏中的各种实体（如植物、僵尸、阳光、豌豆）、
/// 玩家状态（如阳光数量、当前选中的植物）、游戏进程（如是否结束）以及
/// 与游戏核心机制相关的模块（如商店、实体管理器）。
pub struct GameState {
    /// 游戏资源，如图形和声音。
    resources: Resources,
    
    /// 游戏区域的网格布局。
    grid: Grid,
    /// 当前场景中所有植物的集合。
    plants: Vec<Plant>,
    /// 当前场景中所有僵尸的集合。
    zombies: Vec<Zombie>,
    /// 当前场景中所有阳光的集合。
    suns: Vec<Sun>,
    /// 当前场景中所有豌豆的集合。
    peas: Vec<Pea>,
    
    /// 玩家当前的阳光数量。
    sun_count: i32,
    /// 玩家当前从商店选中的待放置植物类型。
    selected_plant: Option<PlantType>,
    /// 标记游戏是否已经结束（游戏失败）。
    game_over: bool,
    /// 标记游戏是否胜利。
    victory: bool,
    /// 标记是否显示"最后一波来袭"消息。
    show_final_wave: bool,
    /// 显示最后一波消息的时间。
    final_wave_message_time: Option<Instant>,
    
    /// 游戏商店，用于购买植物。
    shop: Shop,
    /// 实体管理器，负责生成新的实体，如自然掉落的阳光和来袭的僵尸。
    entity_manager: EntityManager,
    
    /// 游戏当前的状态（运行中、暂停等）
    game_state: crate::core::states::GameState,
    /// 暂停按钮的位置和尺寸
    pause_button_rect: (f32, f32, f32, f32),
    /// 游戏暂停的时间点
    pause_start_time: Option<std::time::Instant>,
}

impl GameState {
    /// 创建并初始化一个新的 `GameState` 实例。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境，用于加载资源等。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，其中包含初始化完成的 `GameState` 实例或者一个错误。
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
            victory: false,
            show_final_wave: false,
            final_wave_message_time: None,
            shop,
            entity_manager,
            game_state: crate::core::states::GameState::InGame,
            pause_button_rect: (750.0, 10.0, 80.0, 40.0), // x, y, width, height
            pause_start_time: None,
        })
    }
}

impl EventHandler for GameState {
    /// 更新游戏状态，此方法会在每一帧被调用。
    ///
    /// 负责处理游戏逻辑的更新，包括实体（植物、僵尸、豌豆、阳光）的状态更新、
    /// 碰撞检测、实体生成、游戏结束条件判断以及商店状态的更新。
    /// 使用固定的时间步长（FIXED_UPDATE_DT_MS）来确保游戏逻辑更新的稳定性。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示更新操作是否成功。
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 设置游戏帧率
        const DESIRED_FPS: u32 = 60;
        // 计算固定的逻辑更新时间步长 (毫秒)
        // (1000ms / FPS) gives ms per frame.
        const FIXED_UPDATE_DT_MS: u64 = (1000.0_f32 / DESIRED_FPS as f32) as u64;

        // ggez::timer::check_update_time 会根据自上次 EventHandler::update 调用以来的时间
        // 来决定逻辑更新循环（while 循环体）需要执行多少次，以达到 DESIRED_FPS。
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.game_over || self.game_state == crate::core::states::GameState::Paused {
                continue;
            }
            
            // 更新阳光
            for sun in &mut self.suns {
                sun.update(FIXED_UPDATE_DT_MS);
            }

            // 更新植物并收集新产生的阳光
            let mut new_suns = Vec::new();
            for plant in &mut self.plants {
                plant.update(FIXED_UPDATE_DT_MS, &mut new_suns, &mut self.peas, &self.zombies); // 传递 self.zombies
            }
            self.suns.append(&mut new_suns);

            // 更新僵尸
            for zombie in &mut self.zombies {
                zombie.update(FIXED_UPDATE_DT_MS);
            }

            // 更新豌豆
            for pea in &mut self.peas {
                pea.update(FIXED_UPDATE_DT_MS);
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

            // 基于时间生成自然阳光
            if self.entity_manager.check_natural_sun_spawn(FIXED_UPDATE_DT_MS) {
                let new_sun = self.entity_manager.spawn_natural_sun();
                self.suns.push(new_sun);
            }

            // 通过关卡控制器更新并生成僵尸，传递当前场上的僵尸信息
            let zombie_spawns = self.entity_manager.update(FIXED_UPDATE_DT_MS, &self.zombies);
            for spawn_info in zombie_spawns {
                let zombie = self.entity_manager.spawn_zombie(spawn_info.zombie_type, spawn_info.row);
                self.zombies.push(zombie);
            }
            
            // 检查是否应该显示最后一波信息
            if self.entity_manager.level_controller.is_final_wave_announced() {
                self.show_final_wave = true;
                self.final_wave_message_time = Some(Instant::now());
            }
            
            // 如果已经显示"最后一波来袭"信息5秒钟，则隐藏它
            if let Some(time) = self.final_wave_message_time {
                if time.elapsed().as_secs() > 5 {
                    self.show_final_wave = false;
                }
            }
            
            // 检查关卡是否胜利完成
            if self.entity_manager.level_controller.is_level_completed(&self.zombies) {
                self.victory = true;
            }
            
            // 更新商店（只有在游戏未暂停时才更新）
            if self.game_state != crate::core::states::GameState::Paused {
                self.shop.update(self.sun_count);
            }
        }

        Ok(())
    }

    /// 绘制游戏画面，此方法会在每一帧的更新之后被调用。
    ///
    /// 通过调用 `Renderer` 模块来绘制游戏背景、网格、所有实体（植物、豌豆、僵尸、阳光）、
    /// UI元素（商店、阳光数量）以及游戏结束画面（如果适用）。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
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
            self.game_over,
            self.victory,
            self.show_final_wave,
            self.game_state,
            self.pause_button_rect
        )
    }

    /// 处理鼠标按键按下事件。
    ///
    /// 当玩家点击鼠标时，此方法被调用。它将事件委托给 `InputHandler` 来处理，
    /// 例如处理阳光的收集、商店卡片的选择、植物的放置或取消选择等操作。
    ///
    /// # Arguments
    ///
    /// * `_ctx` - ggez的上下文环境 (在此方法中未使用)。
    /// * `button` - 被按下的鼠标按键。
    /// * `x` - 鼠标点击位置的x坐标。
    /// * `y` - 鼠标点击位置的y坐标。
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        // 检查是否点击了暂停按钮
        let (btn_x, btn_y, btn_w, btn_h) = self.pause_button_rect;
        if button == MouseButton::Left 
           && x >= btn_x && x <= btn_x + btn_w 
           && y >= btn_y && y <= btn_y + btn_h {
            // 切换游戏状态
            match self.game_state {
                crate::core::states::GameState::Paused => {
                    // 游戏从暂停恢复正常状态
                    self.game_state = crate::core::states::GameState::InGame;
                    
                    // 计算暂停持续了多长时间
                    if let Some(pause_time) = self.pause_start_time {
                        let pause_duration = pause_time.elapsed();
                        
                        // 调整所有卡片的冷却时间
                        for card in &mut self.shop.cards {
                            if let Some(last_used) = card.last_used {
                                // 将卡片的最后使用时间向后推迟暂停持续的时间
                                card.last_used = Some(std::time::Instant::now()
                                    .checked_sub(last_used.elapsed().checked_sub(pause_duration)
                                        .unwrap_or_else(|| std::time::Duration::from_secs(0)))
                                    .unwrap_or_else(std::time::Instant::now));
                                
                                // 恢复暂停前的状态后，立即更新一次冷却显示进度
                                let elapsed = card.last_used.unwrap().elapsed();
                                if elapsed < card.cooldown {
                                    card.cooldown_display_progress = elapsed.as_millis() as f32 / card.cooldown.as_millis() as f32;
                                } else {
                                    card.cooldown_display_progress = 1.0;
                                }
                            }
                        }
                        
                        self.pause_start_time = None;
                    }
                },
                _ => {
                    // 游戏进入暂停状态
                    self.game_state = crate::core::states::GameState::Paused;
                    self.pause_start_time = Some(std::time::Instant::now());
                    
                    // 在暂停时，冷却显示进度已经在update方法中正确设置，此处不需要额外处理
                }
            }
            return;
        }
        
        // 如果游戏暂停，不处理其他输入
        if self.game_state == crate::core::states::GameState::Paused {
            return;
        }
        
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
