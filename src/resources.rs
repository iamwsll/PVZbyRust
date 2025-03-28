use ggez::{Context, GameResult};
use ggez::graphics::{self, Image};
use std::path::Path;

pub struct Resources {
    pub background: Image,
    pub peashooter_images: Vec<Image>,
    pub sunflower_images: Vec<Image>,
    pub zombie_images: Vec<Image>,
    pub sun_image: Image,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        Ok(Resources {
            background: Image::new(ctx, "/other_image/Background.png")?,
            peashooter_images: vec![
                Image::new(ctx, "/plants/Peashooter.png")?,
                // Image::new(ctx, "../../Resource/plants/peashooter2.png")?,
            ],
            sunflower_images: vec![
                Image::new(ctx, "/plants/Sunflower.png")?,
                // Image::new(ctx, "/plants/sunflower2.png")?,
            ],
            zombie_images: vec![
                // Image::new(ctx, "../../Resource/zombies/zombie1.png")?,
                // Image::new(ctx, "../../Resource/zombies/zombie2.png")?,
            ],
            sun_image: Image::new(ctx, "/other_image/Button.png")?,
        })
    }
}
