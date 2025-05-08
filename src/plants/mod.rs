use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};
use crate::resources::Resources;
use crate::grid::{GRID_START_X, GRID_START_Y, GRID_CELL_HEIGHT,GRID_CELL_WIDTH};
use crate::sun::Sun;
use crate::pea::Pea;

// Declare submodules and import their update functions
pub mod peashooter;
pub mod sunflower;
pub mod wallnut;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
}

impl PlantType {
    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::WallNut => 50,
        }
    }
}

pub struct Plant {
    plant_type: PlantType,
    grid_x: usize,
    grid_y: usize,
    health: i32,
    // 动画帧 目前没用?
    animation_frame: usize,
    animation_timer: u64,
    //这个冷却时间是指如豌豆射手的发射时间间隔
    cooldown: u64,
    cooldown_timer: u64,
}

impl Plant {
    pub fn new(plant_type: PlantType, grid_x: usize, grid_y: usize) -> Self {
        let health = match plant_type {
            PlantType::Peashooter => peashooter::INITIAL_HEALTH, // Use specific health
            PlantType::Sunflower => sunflower::INITIAL_HEALTH, // Use specific health
            PlantType::WallNut => wallnut::INITIAL_HEALTH,   // Use specific health
        };

        // 这里的冷却时间是指植物的动作间隔，如向日葵产生阳光的间隔，豌豆射手发射间隔
        let cooldown = match plant_type {
             PlantType::Peashooter => peashooter::COOLDOWN,
             PlantType::Sunflower => sunflower::COOLDOWN,
             _ => 0, // WallNut might not have a cooldown for actions
        };


        Plant {
            plant_type,
            grid_x,
            grid_y,
            health,
            animation_frame: 0,
            animation_timer: 0,
            cooldown, // Use specific cooldown
            cooldown_timer: 0,
        }
    }

    /// 用来更新植物状态
    /// @param dt: 距离上次更新的时间
    /// @param suns: 由于向日葵会产生阳光，所以需要传入阳光的引用
    /// @param peas: 由于豌豆射手会发射豌豆，所以需要传入豌豆的引用
    /// @return: None
    /// @note: 添加植物时需要记得在这里修改有关信息。
    pub fn update(&mut self, dt: u64, suns: &mut Vec<Sun>, peas: &mut Vec<Pea>) {
        // 动画更新 (通用逻辑)
        self.animation_timer += dt;
        if self.animation_timer > 100 { // 每100ms更新一次帧动画
            // 根据植物类型确定动画帧数
            let frame_count = match self.plant_type {
                PlantType::Sunflower => 18, // 向日葵有18帧
                PlantType::Peashooter => 13, // 豌豆射手13帧
                PlantType::WallNut => 16,   // 坚果墙16帧
                // 如果未来添加更多植物，在这里添加它们的帧数
            };
            if frame_count > 0 {
                self.animation_frame = (self.animation_frame + 1) % frame_count;
            }
            self.animation_timer = 0;
        }

        // 冷却更新和动作执行 (委托给子模块)
        if self.cooldown > 0 {
            self.cooldown_timer += dt;
            if self.cooldown_timer >= self.cooldown {
                self.cooldown_timer = 0; // 重置计时器

                // 调用特定植物的 update 函数
                match self.plant_type {
                    PlantType::Sunflower => {
                        // 向日葵需要传入suns
                        sunflower::update(self.grid_x, self.grid_y, suns);
                    },
                    PlantType::Peashooter => {
                        // 豌豆射手需要传入peas
                        peashooter::update(self.grid_x, self.grid_y, peas);
                    },
                    PlantType::WallNut => wallnut::update(self.grid_x, self.grid_y),
                };
            }
        }

        // TODO: 添加特定更新逻辑调用，例如坚果墙的损坏状态检查 (可以在 wallnut::update 中实现)
    }

    /// 辅助用的抽象函数：获取当前植物动画帧对应的图像
    /// @param resources: 游戏资源引用
    /// @return: 对图像的引用
    fn get_current_frame_image<'a>(&self, resources: &'a Resources) -> &'a graphics::ImageGeneric<graphics::GlBackendSpec> {
        // 根据植物类型获取对应的图像列表、备用图像和名称
        let (images, fallback_image, plant_name) = match self.plant_type {
            PlantType::Peashooter => (&resources.peashooter_images, &resources.peashooter_card, "Peashooter"),
            PlantType::Sunflower => (&resources.sunflower_images, &resources.sunflower_card, "Sunflower"),
            PlantType::WallNut => (&resources.wallnut_images, &resources.wallnut_card, "WallNut"),
            // 如果未来添加更多植物，在这里添加它们的图像资源
            // _ => (&resources.default_images, &resources.default_card, "Unknown Plant"), // 示例：处理未知类型
        };

        let frame_count = images.len();
        if frame_count > 0 {
            // 确保动画帧索引在有效范围内
            &images[self.animation_frame % frame_count]
        } else {
            // 如果图像列表为空，打印警告并返回备用图像
            println!("Warning: {} images not loaded or empty!", plant_name);
            fallback_image
        }
    }

    pub fn draw(&self, ctx: &mut Context, resources: &Resources) -> GameResult {
        // 计算植物在屏幕上的位置,最后一项是偏移量 (Common logic)
        let x = GRID_START_X + (self.grid_x as f32) * GRID_CELL_WIDTH + GRID_CELL_WIDTH / 4.0;
        let y = GRID_START_Y + (self.grid_y as f32) * GRID_CELL_HEIGHT+ GRID_CELL_HEIGHT / 4.0;

        // 使用辅助函数获取当前帧的图像
        let image = self.get_current_frame_image(resources);

        // 绘制图像
        graphics::draw(
            ctx,
            image,
            DrawParam::default()
                .dest([x, y])
                .scale([0.8, 0.8]), // 考虑是否需要根据植物类型调整缩放?
        )
    }
}
