//! # 商店模块 (`shop`)
//!
//! 管理游戏中的植物商店界面，允许玩家选择和购买植物。
//! 商店中展示了不同植物的卡片，每张卡片显示植物的图像、阳光成本，
//! 并处理购买后的冷却逻辑。

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Color, Rect};
use crate::plants::{PlantType, PlantFactory};
use crate::core::resources::Resources;
use std::time::{Duration, Instant};

/// 商店中植物卡片的宽度。
pub const CARD_WIDTH: f32 = 50.0;
/// 商店中植物卡片的高度。
pub const CARD_HEIGHT: f32 = 80.0;
/// 商店区域在屏幕上的起始X坐标。
pub const SHOP_START_X: f32 = 325.0;
/// 商店区域在屏幕上的起始Y坐标。
pub const SHOP_START_Y: f32 = 8.0;
/// 商店中相邻植物卡片之间的水平间距。
pub const CARD_SPACING: f32 = 10.0;

/// 不同植物类型在商店中的冷却时间（毫秒）。
/// 顺序应与 `PlantType` 枚举的定义顺序一致。
const COOLDOWN_TIMES: [u64; 3] = [
    7500,  // 豌豆射手 (Peashooter)
    5000,  // 向日葵 (Sunflower)
    25000, // 坚果墙 (WallNut)
];

/// 代表商店中的一张植物卡片。
///
/// 每张卡片关联一种植物类型，并管理其可用性（基于阳光和冷却时间）。
pub struct PlantCard {
    /// 卡片对应的植物类型。
    pub plant_type: PlantType,
    /// 卡片在屏幕上的位置 (x, y)。
    pub position: (f32, f32),
    /// 卡片当前是否可用（可购买）。
    pub available: bool,
    /// 购买此植物后的冷却时间。
    pub cooldown: Duration,
    /// 上次购买此植物的时间点，用于计算冷却。
    pub last_used: Option<Instant>,
    /// 卡片的矩形区域，用于碰撞检测（点击）。
    pub rect: Rect,
}

impl PlantCard {
    /// 创建一个新的 `PlantCard` 实例。
    ///
    /// # Arguments
    ///
    /// * `plant_type` - 此卡片代表的植物类型。
    /// * `index` - 卡片在商店显示中的索引，用于计算其水平位置。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `PlantCard`。
    pub fn new(plant_type: PlantType, index: usize) -> Self {
        let x = SHOP_START_X + (CARD_WIDTH + CARD_SPACING) * index as f32;
        let y = SHOP_START_Y;
        
        PlantCard {
            plant_type,
            position: (x, y),
            available: true,
            cooldown: Duration::from_millis(COOLDOWN_TIMES[plant_type as usize]),
            last_used: None,
            rect: Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT),
        }
    }

    /// 更新植物卡片的状态，主要是其可用性。
    ///
    /// 卡片的可用性取决于：
    /// 1. 是否在冷却时间内。
    /// 2. 玩家当前的阳光数量是否足够支付植物的成本。
    ///
    /// # Arguments
    ///
    /// * `sun_count` - 玩家当前的阳光数量。
    pub fn update(&mut self, sun_count: i32) {
        // 检查冷却时间
        if let Some(last_used) = self.last_used {
            if last_used.elapsed() < self.cooldown {
                self.available = false;
            } else {
                self.available = sun_count >= self.plant_type.cost();
            }
        } else {
            self.available = sun_count >= self.plant_type.cost();
        }
    }

    /// 在屏幕上绘制植物卡片。
    ///
    /// 包括绘制植物图像、阳光成本，以及在冷却时显示遮罩。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的 `Context` 引用。
    /// * `resources` - 游戏资源 (`Resources`) 的引用，用于获取图像。
    ///
    /// # Returns
    ///
    /// 如果绘制成功，返回 `GameResult<()>`。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 创建一个临时的植物实例来获取卡片图像
        let plant = PlantFactory::create_plant(self.plant_type);
        let card_image = plant.get_card_image(resources);
        
        // 绘制卡片
        graphics::draw(
            ctx,
            card_image,
            DrawParam::default()
                .dest([self.position.0, self.position.1])
                .scale([0.9, 0.9])
        )?;
        
        // 如果卡片在冷却中，绘制半透明遮罩
        if let Some(last_used) = self.last_used {
            let elapsed = last_used.elapsed();
            if elapsed < self.cooldown {
                let cooldown_ratio = elapsed.as_millis() as f32 / self.cooldown.as_millis() as f32;
                let mask_height = CARD_HEIGHT * (1.0 - cooldown_ratio);
                
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(self.position.0, self.position.1, CARD_WIDTH, mask_height),
                    Color::new(0.0, 0.0, 0.0, 0.5),
                )?;
                
                graphics::draw(ctx, &rect, DrawParam::default())?;
            }
        }
        
        // 绘制阳光消耗
        let cost_text = graphics::Text::new(
            graphics::TextFragment::new(self.plant_type.cost().to_string())
                .color(Color::BLACK)
                .scale(15.0)
        );
        
        let text_pos = [
            self.position.0 + CARD_WIDTH/2.0 - cost_text.width(ctx) as f32 / 2.0,
            self.position.1 + CARD_HEIGHT - 18.0
        ];
        
        graphics::draw(ctx, &cost_text, DrawParam::default().dest(text_pos))?;
        
        Ok(())
    }

    /// 检查给定的屏幕坐标是否在该卡片的矩形区域内。
    ///
    /// # Arguments
    ///
    /// * `x` - 要检查的X坐标。
    /// * `y` - 要检查的Y坐标。
    ///
    /// # Returns
    ///
    /// 如果坐标点在卡片内，则返回 `true`，否则返回 `false`。
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains([x, y])
    }

    /// 标记卡片已被使用（购买），并开始冷却计时。
    pub fn use_card(&mut self) {
        self.last_used = Some(Instant::now());
    }
}

/// 代表游戏中的植物商店。
///
/// `Shop` 结构体包含一个植物卡片列表，并跟踪当前是否有选中的植物类型
/// （即玩家点击了卡片但尚未放置植物）。
pub struct Shop {
    /// 商店中所有植物卡片的向量。
    pub cards: Vec<PlantCard>,
    /// 当前从商店选择的植物类型（如果有）。
    pub selected_plant: Option<PlantType>,
}

impl Shop {
    /// 创建一个新的 `Shop` 实例。
    ///
    /// 初始化时，会为游戏中定义的所有可购买植物类型创建 `PlantCard`。
    ///
    /// # Returns
    ///
    /// 返回一个新的 `Shop` 实例。
    pub fn new() -> Self {
        let mut cards = Vec::new();
        
        // 添加植物卡片
        cards.push(PlantCard::new(PlantType::Sunflower, 0));
        cards.push(PlantCard::new(PlantType::Peashooter, 1));
        cards.push(PlantCard::new(PlantType::WallNut, 2));
        
        Shop {
            cards,
            selected_plant: None,
        }
    }
    
    /// 更新商店中所有植物卡片的状态。
    ///
    /// 此方法会遍历所有卡片并调用它们各自的 `update` 方法，
    /// 以根据当前阳光数量和冷却状态更新其可用性。
    ///
    /// # Arguments
    ///
    /// * `sun_count` - 玩家当前的阳光数量。
    pub fn update(&mut self, sun_count: i32) {
        for card in &mut self.cards {
            card.update(sun_count);
        }
    }
    
    /// 在屏幕上绘制整个商店界面。
    ///
    /// 包括绘制所有植物卡片，以及如果玩家已选择植物，
    /// 则绘制一个跟随鼠标的半透明植物预览图像。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的 `Context` 引用。
    /// * `resources` - 游戏资源 (`Resources`) 的引用。
    ///
    /// # Returns
    ///
    /// 如果绘制成功，返回 `GameResult<()>`。
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        for card in &self.cards {
            card.draw(ctx, resources)?;
        }
        
        // 如果有选择的植物，绘制跟随鼠标的植物预览
        if let Some(plant_type) = self.selected_plant {
            let mouse_pos = ggez::input::mouse::position(ctx);
            let x = mouse_pos.x;
            let y = mouse_pos.y;
            
            // 创建一个临时的植物实例来获取预览图像
            let plant = PlantFactory::create_plant(plant_type);
            let image = plant.get_current_frame_image(resources, 0); // 使用第一帧作为预览
            
            graphics::draw(
                ctx,
                image,
                DrawParam::default()
                    .dest([x - 30.0, y - 30.0])
                    .scale([0.6, 0.6])
                    .color(Color::new(1.0, 1.0, 1.0, 0.7)),
            )?;
        }
        
        Ok(())
    }
    
    /// 处理在商店区域的鼠标点击事件。
    ///
    /// 如果当前没有选中的植物，此方法会检查点击是否落在某个可用的植物卡片上。
    /// 如果是，并且玩家阳光充足，则将该植物类型标记为 `selected_plant`，
    /// 触发卡片的冷却，并返回选中的植物类型。
    /// 如果当前已有选中的植物，则任何点击都会取消该选择。
    ///
    /// # Arguments
    ///
    /// * `x` - 鼠标点击的X坐标。
    /// * `y` - 鼠标点击的Y坐标。
    /// * `sun_count` - 玩家当前的阳光数量。
    ///
    /// # Returns
    ///
    /// 如果成功选择了一个植物类型，则返回 `Some(PlantType)`。
    /// 如果点击未导致选择（例如，点击空白区域、阳光不足、卡片冷却中，或取消选择），
    /// 则返回 `None`。
    pub fn handle_click(&mut self, x: f32, y: f32, sun_count: i32) -> Option<PlantType> {
        // 如果有选中的植物，就取消选择
        if self.selected_plant.is_some() {
            self.selected_plant = None;
            return None;
        }
        
        // 否则检查是否点击了某个卡片
        for card in &mut self.cards {
            if card.contains_point(x, y) && card.available {
                let plant_type = card.plant_type;
                if sun_count >= plant_type.cost() {
                    self.selected_plant = Some(plant_type);
                    card.use_card();
                    return Some(plant_type);
                }
            }
        }
        
        None
    }
}
