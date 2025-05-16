//! # 渲染模块
//!
//! `renderer` 模块负责游戏的所有渲染任务。
//! 它包含了将游戏状态（如植物、僵尸、子弹、UI元素等）绘制到屏幕上的所有逻辑。

use crate::core::resources::Resources;
use crate::ui::grid::Grid;
use crate::plants::Plant;
use crate::entities::pea::Pea;
use crate::zombies::Zombie;
use crate::entities::sun::Sun;
use crate::ui::shop::Shop;
use crate::ui::shovel::Shovel;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text, TextFragment};

/// 渲染器结构体，封装了所有与游戏场景绘制相关的操作。
///
/// `Renderer` 是一个无状态的结构体，其方法通常接收绘图上下文 (`Context`)、
/// 游戏资源 (`Resources`) 以及需要绘制的游戏实体和UI元素作为参数。
pub struct Renderer;

impl Renderer {
    /// 绘制完整的游戏场景。
    ///
    /// 此函数是主要的绘制入口，它会按顺序调用其他辅助绘制函数来渲染游戏的各个层面，
    /// 包括背景、网格、植物、豌豆、僵尸、阳光、UI元素，以及游戏结束画面（如果适用）。
    /// 最后，它将所有绘制内容呈现到屏幕上。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境，用于绘图操作。
    /// * `resources` - 包含所有已加载图像资源的 `Resources` 实例。
    /// * `grid` - 游戏网格，用于辅助定位和可能的调试绘制。
    /// * `plants` - 当前场景中所有植物的切片。
    /// * `peas` - 当前场景中所有豌豆的切片。
    /// * `zombies` - 当前场景中所有僵尸的切片。
    /// * `suns` - 当前场景中所有阳光的切片。
    /// * `shop` - 游戏商店实例，用于绘制商店UI。
    /// * `sun_count` - 当前玩家拥有的阳光数量。
    /// * `game_over` - 一个布尔值，指示游戏是否已结束。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    pub fn draw_game(
        ctx: &mut Context, 
        resources: &Resources,
        grid: &Grid,
        plants: &[Plant],
        peas: &[Pea],
        zombies: &[Zombie],
        suns: &[Sun],
        shop: &Shop,
        sun_count: i32,
        game_over: bool,
        victory: bool,
        show_final_wave: bool,
        game_state: crate::core::states::GameState,
        pause_button_rect: (f32, f32, f32, f32),
        shovel: &Shovel,
        is_initial_pause: bool
    ) -> GameResult {
        // 清空屏幕
        graphics::clear(ctx, Color::WHITE);

        // 绘制背景
        Renderer::draw_background(ctx, resources)?;
        
        // 绘制网格（调试用）
        grid.draw(ctx)?;
        
        // 绘制植物
        for plant in plants {
            plant.draw(ctx, resources)?;
        }
        
        // 绘制豌豆
        for pea in peas {
            pea.draw(ctx, resources)?;
        }
        
        // 绘制僵尸
        for zombie in zombies {
            zombie.draw(ctx, resources)?;
        }
        
        // 绘制阳光
        for sun in suns {
            sun.draw(ctx, resources)?;
        }
        
        // 绘制UI元素
        Renderer::draw_ui(ctx, resources, shop, sun_count)?;
        
        // 绘制铲子
        shovel.draw(ctx, resources)?;
        
        // 绘制暂停按钮
        let (x, y, w, h) = pause_button_rect;
        let button_text = match game_state {
            crate::core::states::GameState::Paused => if is_initial_pause { "Start" } else { "Continue" },
            _ => "pause"
        };
        
        // 绘制按钮背景
        graphics::draw(
            ctx,
            &resources.button_image,
            DrawParam::default()
                .dest([x, y])
                .scale([w / resources.button_image.width() as f32, h / resources.button_image.height() as f32])
        )?;
        
        // 绘制按钮文字
        let button_text = Text::new(
            TextFragment::new(button_text)
                .color(Color::BLACK)
                .scale(20.0)
        );
        
        let text_width = button_text.width(ctx);
        let text_height = button_text.height(ctx);
        
        graphics::draw(
            ctx,
            &button_text,
            DrawParam::default().dest([
                x + w / 2.0 - text_width / 2.0,
                y + h / 2.0 - text_height / 2.0,
            ])
        )?;
        
        // 如果游戏暂停，显示暂停信息
        if game_state == crate::core::states::GameState::Paused {
            Renderer::draw_pause_message(ctx, is_initial_pause)?;
        }
        
        // 如果显示最后一波信息
        if show_final_wave {
            Renderer::draw_final_wave_message(ctx)?;
        }
        
        // 如果游戏结束，显示结束画面
        if game_over {
            Renderer::draw_game_over(ctx)?;
        }
        
        // 如果游戏胜利，显示胜利画面
        if victory {
            Renderer::draw_victory_message(ctx)?;
        }
        
        // 呈现画面
        graphics::present(ctx)?;
        
        Ok(())
    }
    
    /// 绘制游戏背景，包括主背景图和商店面板。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `resources` - 包含背景和商店图像的 `Resources` 实例。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_background(ctx: &mut Context, resources: &Resources) -> GameResult {
        // 绘制主背景
        graphics::draw(ctx, &resources.background, DrawParam::default())?;
        
        // 绘制商店面板
        graphics::draw(
            ctx, 
            &resources.shop_image, 
            DrawParam::default().dest([250.0, 0.0])
        )?;
        
        Ok(())
    }
    
    /// 绘制用户界面（UI）元素，主要包括商店和阳光数量显示。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `resources` - `Resources` 实例，主要用于商店绘制时传递给商店的 `draw` 方法。
    /// * `shop` - 游戏商店实例。
    /// * `sun_count` - 当前玩家拥有的阳光数量，用于显示。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_ui(ctx: &mut Context, resources: &Resources, shop: &Shop, sun_count: i32) -> GameResult {
        // 绘制商店卡片和选中植物预览
        shop.draw(ctx, resources)?;
        
        // 绘制阳光数量
        let sun_text = Text::new(
            TextFragment::new(format!("{}", sun_count))
                .color(Color::BLACK)
                .scale(25.0)
        );
        
        graphics::draw(
            ctx,
            &sun_text,
            DrawParam::default().dest([280.0, 65.0])
        )?;
        
        Ok(())
    }
    
    /// 绘制游戏结束画面。
    ///
    /// 当游戏结束时，在屏幕中央显示 "GAME OVER" 文本。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_game_over(ctx: &mut Context) -> GameResult {
        let game_over_text = Text::new(
            TextFragment::new("GAME OVER")
                .color(Color::RED)
                .scale(100.0)
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
            ])
        )?;
        
        Ok(())
    }
    
    /// 绘制最后一波来袭消息。
    ///
    /// 当最后一波僵尸出现时，在屏幕中央显示 "最后一波来袭!" 文本。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_final_wave_message(ctx: &mut Context) -> GameResult {
        let final_wave_text = Text::new(
            TextFragment::new("The final wave is coming!")
                .color(Color::RED)
                .scale(70.0)
        );
        
        let text_width = final_wave_text.width(ctx);
        let text_height = final_wave_text.height(ctx);
        let screen_size = graphics::drawable_size(ctx);
        
        graphics::draw(
            ctx,
            &final_wave_text,
            DrawParam::default().dest([
                screen_size.0 / 2.0 - text_width / 2.0,
                screen_size.1 / 2.0 - text_height / 2.0,
            ])
        )?;
        
        Ok(())
    }
    
    /// 绘制游戏胜利画面。
    ///
    /// 当玩家成功击败所有僵尸后，在屏幕中央显示 "游戏胜利!" 文本。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_victory_message(ctx: &mut Context) -> GameResult {
        let victory_text = Text::new(
            TextFragment::new("you win!")
                .color(Color::GREEN)
                .scale(100.0)
        );
        
        let text_width = victory_text.width(ctx);
        let text_height = victory_text.height(ctx);
        let screen_size = graphics::drawable_size(ctx);
        
        graphics::draw(
            ctx,
            &victory_text,
            DrawParam::default().dest([
                screen_size.0 / 2.0 - text_width / 2.0,
                screen_size.1 / 2.0 - text_height / 2.0,
            ])
        )?;
        
        Ok(())
    }
    
    /// 绘制游戏暂停信息。
    ///
    /// 当游戏处于暂停状态时，在屏幕中央显示暂停文本。
    /// 初始暂停时显示"请确保屏幕缩放比例为100%\n点击右上角开始"
    /// 普通暂停时显示"Game Paused"
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    /// * `is_initial_pause` - 是否为初始暂停状态。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，表示绘制操作是否成功。
    fn draw_pause_message(ctx: &mut Context, is_initial_pause: bool) -> GameResult {
        let pause_text = if is_initial_pause {
            Text::new(
                TextFragment::new("Make sure the screen zoom ratio is 100%.\nClick Start in the upper right corner.")
                    .color(Color::RED)
                    .scale(60.0)
            )
        } else {
            Text::new(
                TextFragment::new("Game Paused")
                    .color(Color::BLUE)
                    .scale(80.0)
            )
        };
        
        let text_width = pause_text.width(ctx);
        let text_height = pause_text.height(ctx);
        let screen_size = graphics::drawable_size(ctx);
        
        graphics::draw(
            ctx,
            &pause_text,
            DrawParam::default().dest([
                screen_size.0 / 2.0 - text_width / 2.0,
                screen_size.1 / 2.0 - text_height / 2.0,
            ])
        )?;
        
        Ok(())
    }
}