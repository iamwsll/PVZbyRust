use ggez::{Context, GameResult};
use ggez::graphics::{self, Image};
use std::path::Path;

pub struct Resources {
    pub background: Image,
    pub shop_image: Image, // 添加商店面板图片
    pub peashooter_images: Vec<Image>,
    pub sunflower_images: Vec<Image>,
    pub wallnut_images: Vec<Image>, // 添加坚果墙图片
    pub zombie_images: Vec<Image>,
    pub sun_images: Vec<Image>,  // 改为数组以支持动画
    // 添加植物卡片图像
    pub peashooter_card: Image,
    pub sunflower_card: Image,
    pub wallnut_card: Image,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        // 创建一个向量来存储阳光的所有帧
        let mut sun_images = Vec::with_capacity(22);
        
        // 循环加载22帧阳光图片
        for i in 0..=21 {
            sun_images.push(Image::new(ctx, &format!("/other_image/sun/Sun{}.png", i))?);
        }
        
        Ok(Resources {
            background: Image::new(ctx, "/other_image/Background.png")?,
            shop_image: Image::new(ctx, "/other_image/Shop.png")?,
            peashooter_images: vec![
                Image::new(ctx, "/plants/Peashooter.png")?,
                // Image::new(ctx, "../../Resource/plants/peashooter2.png")?,
            ],
            sunflower_images: vec![
                Image::new(ctx, "/plants/Sunflower.png")?,
                // Image::new(ctx, "/plants/sunflower2.png")?,
            ],
            wallnut_images: vec![
                Image::new(ctx, "/plants/WallNut.png")?,
                // Image::new(ctx, "/plants/wallnut2.png")?,
            ],
            zombie_images: vec![
                // Image::new(ctx, "../../Resource/zombies/zombie1.png")?,
                // Image::new(ctx, "../../Resource/zombies/zombie2.png")?,
            ],
            sun_images, // 使用加载的22帧图片
            
            // 加载植物卡片图像
            peashooter_card: Image::new(ctx, "/plants/Peashooter.png")?,
            sunflower_card: Image::new(ctx, "/plants/Sunflower.png")?,
            wallnut_card: Image::new(ctx, "/plants/WallNut.png")?,
        })
    }
}
