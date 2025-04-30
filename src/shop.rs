use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Color, Rect};
use ggez::input::mouse::MouseButton;
use crate::plants::PlantType;
use crate::resources::Resources;
use std::time::{Duration, Instant};

// 卡片在商店中的尺寸
pub const CARD_WIDTH: f32 = 50.0;
pub const CARD_HEIGHT: f32 = 80.0;
pub const SHOP_START_X: f32 = 325.0;
pub const SHOP_START_Y: f32 = 8.0;
pub const CARD_SPACING: f32 = 10.0;

// 卡片冷却时间（毫秒）
const COOLDOWN_TIMES: [u64; 3] = [
    5000,  // Peashooter
    5000,  // Sunflower
    10000,  // WallNut
];

pub struct PlantCard {
    pub plant_type: PlantType,
    pub position: (f32, f32),
    pub available: bool,
    pub cooldown: Duration,
    pub last_used: Option<Instant>,
    pub rect: Rect,
}

impl PlantCard {
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

    pub fn update(&mut self, sun_count: i32) {
        // 检查冷却时间 这里额外处理了第一次使用时的情况
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

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 绘制卡片背景
        let card_image = match self.plant_type {
            PlantType::Peashooter => &resources.peashooter_card,
            PlantType::Sunflower => &resources.sunflower_card,
            PlantType::WallNut => &resources.wallnut_card,
        };
        
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

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rect.contains([x, y])
    }

    pub fn use_card(&mut self) {
        self.last_used = Some(Instant::now());
    }
}

pub struct Shop {
    pub cards: Vec<PlantCard>,
    pub selected_plant: Option<PlantType>,
}

impl Shop {
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
    
    pub fn update(&mut self, sun_count: i32) {
        for card in &mut self.cards {
            card.update(sun_count);
        }
    }
    
    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        for card in &self.cards {
            card.draw(ctx, resources)?;
        }
        
        // 如果有选择的植物，绘制跟随鼠标的植物预览
        if let Some(plant_type) = self.selected_plant {
            let mouse_pos = ggez::input::mouse::position(ctx);
            let x = mouse_pos.x;
            let y = mouse_pos.y;
            let image = match plant_type {
                PlantType::Peashooter => &resources.peashooter_images[0],
                PlantType::Sunflower => &resources.sunflower_images[0],
                PlantType::WallNut => &resources.wallnut_images[0],
            };
            
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
