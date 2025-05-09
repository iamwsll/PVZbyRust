use crate::core::resources::Resources;
use crate::ui::grid::Grid;
use crate::plants::Plant;
use crate::entities::pea::Pea;
use crate::zombies::Zombie;
use crate::entities::sun::Sun;
use crate::ui::shop::Shop;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Text, TextFragment};

/// 渲染管理模块，负责处理游戏中的所有绘制逻辑
pub struct Renderer;

impl Renderer {
    /// 绘制整个游戏画面
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
        game_over: bool
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
        
        // 如果游戏结束，显示结束画面
        if game_over {
            Renderer::draw_game_over(ctx)?;
        }
        
        // 呈现画面
        graphics::present(ctx)?;
        
        Ok(())
    }
    
    /// 绘制背景
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
    
    /// 绘制UI元素
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
            DrawParam::default().dest([285.0, 65.0])
        )?;
        
        Ok(())
    }
    
    /// 绘制游戏结束画面
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
}