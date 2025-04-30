use ggez::graphics::{Image, Font};
use ggez::{Context, GameResult};
use std::collections::HashMap; // Keep HashMap if needed for other resources
use std::path::Path;

pub struct Resources {
    // Background and UI
    pub background: Image,
    pub shop_image: Image,
    pub sun_images: Vec<Image>, // Keep sun animation frames

    // Plant Images (Individual and Card)
    pub peashooter_images: Vec<Image>,
    pub sunflower_images: Vec<Image>,
    pub wallnut_images: Vec<Image>,
    pub peashooter_card: Image,
    pub sunflower_card: Image,
    pub wallnut_card: Image,

    // Zombie Images (Add different states)
    pub zombie_walk_images: Vec<Image>, // For walking animation
    // pub zombie_attack_images: Vec<Image>, // Future: For attacking animation
    // pub zombie_die_images: Vec<Image>, // Future: For dying animation
    // Add images for other zombie types (Conehead, Buckethead) here...

    // Projectile Images
    // pub pea_image: Image, // Future: For peashooter projectile
    // pub snow_pea_image: Image, // Future: For snow pea projectile

    // Font (Optional, if needed for custom text rendering)
    // pub font: Font,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        // Load background and UI
        let background = Image::new(ctx, "/other_image/Background.png")?;
        let shop_image = Image::new(ctx, "/other_image/Shop.png")?;

        // Load sun animation frames
        let mut sun_images = Vec::new();
        for i in 0..=21 {
            let path = format!("/other_image/sun/Sun{}.png", i);
            if Path::new("Resource").join(path.trim_start_matches('/')).exists() {
                 sun_images.push(Image::new(ctx, &path)?);
            } else {
                 println!("Warning: Sun image not found: {}", path);
            }
        }
         if sun_images.is_empty() {
             println!("Warning: No sun images loaded. Using fallback.");
             // Add a fallback mechanism if needed, e.g., load a default image or use a colored rect
             // For now, we'll let it potentially panic later if used without images.
         }


        // Load plant images (assuming simple animation or single frame for now)
        // TODO: Load proper animation sequences if available
        let peashooter_images = vec![
            Image::new(ctx, "/plants/Peashooter.gif")?, // Assuming GIF might load first frame or need specific handling
            // Add more frames if available, e.g., Image::new(ctx, "/plants/Peashooter_frame2.png")?
        ];
        let sunflower_images = vec![
            Image::new(ctx, "/plants/SunFlower.gif")?,
             // Add more frames...
        ];
        let wallnut_images = vec![
            Image::new(ctx, "/plants/WallNut.gif")?,
            // Add more frames for damaged states...
            // Image::new(ctx, "/plants/WallNut1.gif")?,
            // Image::new(ctx, "/plants/WallNut2.gif")?,
        ];

        // Load plant cards
        let peashooter_card = Image::new(ctx, "/plants/Peashooter.png")?; // Assuming card uses the static PNG
        let sunflower_card = Image::new(ctx, "/plants/SunFlower.png")?;
        let wallnut_card = Image::new(ctx, "/plants/WallNut.png")?;


        // Load zombie images
        // TODO: Load proper animation sequences
        let zombie_walk_images = vec![
            Image::new(ctx, "/zombies/ZombieWalk1.gif")?,
            Image::new(ctx, "/zombies/ZombieWalk2.gif")?,
        ];
        // let zombie_attack_images = vec![Image::new(ctx, "/zombies/ZombieAttack.gif")?];
        // let zombie_die_images = vec![Image::new(ctx, "/zombies/ZombieDie.gif")?];


        Ok(Resources {
            background,
            shop_image,
            sun_images,
            peashooter_images,
            sunflower_images,
            wallnut_images,
            peashooter_card,
            sunflower_card,
            wallnut_card,
            zombie_walk_images,
            // zombie_attack_images,
            // zombie_die_images,
        })
    }
}
