use ggez::graphics::{Image, Font};
use ggez::{Context, GameResult};
use std::collections::HashMap; // Keep HashMap if needed for other resources
use std::path::Path;
use std::vec;

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
    // pub zombie_walk_images: Vec<Image>, // For walking animation
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

        // Load plant images
        let mut peashooter_images = Vec::new(); // Placeholder for Peashooter images
        for i in 1..=13 { // Loop from 1 to 13
            let path = format!("/plants/Peashooter/{}.png", i);
            // Check if the file exists before trying to load it
            // Note: Adjust the base path ("Resource") if your structure differs
            if Path::new("Resource").join(path.trim_start_matches('/')).exists() {
                 match Image::new(ctx, &path) {
                     Ok(img) => peashooter_images.push(img),
                     Err(e) => println!("Warning: Failed to load peashooter image {}: {}", path, e),
                 }
            } else {
                 println!("Warning: Peashooter image not found: {}", path);
            }
        }

        let mut sunflower_images = Vec::new();
        for i in 1..=18 { // Loop from 1 to 18
            let path = format!("/plants/SunFlower/{}.png", i);
            // Check if the file exists before trying to load it
            // Note: Adjust the base path ("Resource") if your structure differs
            if Path::new("Resource").join(path.trim_start_matches('/')).exists() {
                 match Image::new(ctx, &path) {
                     Ok(img) => sunflower_images.push(img),
                     Err(e) => println!("Warning: Failed to load sunflower image {}: {}", path, e),
                 }
            } else {
                 println!("Warning: Sunflower image not found: {}", path);
            }
        }


        let mut wallnut_images = Vec::new();
        for i in 1..=16 { // Loop from 1 to 16 (assuming only one image)
            let path = format!("/plants/WallNut/WallnutFull/{}.png", i);
            // Check if the file exists before trying to load it
            // Note: Adjust the base path ("Resource") if your structure differs
            if Path::new("Resource").join(path.trim_start_matches('/')).exists() {
                 match Image::new(ctx, &path) {
                     Ok(img) => wallnut_images.push(img),
                     Err(e) => println!("Warning: Failed to load wallnut image {}: {}", path, e),
                 }
            } else {
                 println!("Warning: Wallnut image not found: {}", path);
            }
        }

        // Load plant cards
        let peashooter_card = Image::new(ctx, "/plants/Peashooter.png")?;
        let sunflower_card = Image::new(ctx, "/plants/SunFlower.png")?;
        let wallnut_card = Image::new(ctx, "/plants/WallNut.png")?;


        // // Load zombie images
        // // TODO: Load proper animation sequences
        // let zombie_walk_images = vec![
        //     Image::new(ctx, "/zombies/ZombieWalk1.gif")?,
        //     Image::new(ctx, "/zombies/ZombieWalk2.gif")?,
        // ];
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
            // zombie_walk_images, // Keep commented if not loaded yet
            // zombie_attack_images,
            // zombie_die_images,
            // Assign other potentially unloaded Vecs as empty or handle appropriately
        })
    }
}
