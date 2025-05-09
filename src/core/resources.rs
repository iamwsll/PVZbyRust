use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::path::Path;

pub struct Resources {
    // 背景和ui
    pub background: Image,
    pub shop_image: Image,
    pub sun_images: Vec<Image>, // Keep sun animation frames

    // 植物图像，包括动画帧和商店中的card
    pub peashooter_images: Vec<Image>,
    pub sunflower_images: Vec<Image>,
    pub wallnut_images: Vec<Image>,
    pub peashooter_card: Image,
    pub sunflower_card: Image,
    pub wallnut_card: Image,

    // 僵尸图像
    pub zombies_walk1_images: Vec<Image>, //僵尸行走动作1
    pub zombie_attack_images: Vec<Image>, // 僵尸攻击动画
    pub zombie_die_images: Vec<Image>, // 僵尸死亡动作
    pub zombie_head_images: Vec<Image>, // 僵尸头掉落动作
    // Add images for other zombie types (Conehead, Buckethead) here...

    // 豌豆射手的豌豆图像
    pub pea_image: Image,     // 普通豌豆
    // pub pea_snow_image: Image, // 寒冰豌豆

    // Font (Optional, if needed for custom text rendering)
    // pub font: Font,
}

// 加载动画帧的帮助函数
fn load_animation_frames(
    ctx: &mut Context,
    path_pattern: &str,
    frame_count: std::ops::RangeInclusive<usize>,
    asset_name: &str,
) -> GameResult<Vec<Image>> {
    let mut frames = Vec::new();
    for i in frame_count {
        let path = path_pattern.replace("{}", &i.to_string());
        let full_path = Path::new("Resource").join(path.trim_start_matches('/'));
        if full_path.exists() {
            match Image::new(ctx, &path) {
                Ok(img) => frames.push(img),
                Err(e) => println!(
                    "Warning: Failed to load {} image {}: {}",
                    asset_name, path, e
                ),
            }
        } else {
            println!("Warning: {} image not found: {}", asset_name, path);
        }
    }
    Ok(frames)
}


impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        // Load background and UI
        let background = Image::new(ctx, "/other_image/Background.png")?;
        let shop_image = Image::new(ctx, "/other_image/Shop.png")?;

        // 加载太阳动画帧
        let sun_images = load_animation_frames(ctx, "/other_image/sun/Sun{}.png", 0..=21, "Sun")?;

        //加载植物图像
        let peashooter_images = load_animation_frames(ctx, "/plants/Peashooter/{}.png", 1..=13, "Peashooter")?;

        let sunflower_images = load_animation_frames(ctx, "/plants/SunFlower/{}.png", 1..=18, "Sunflower")?;

        let wallnut_images = load_animation_frames(ctx, "/plants/WallNut/WallnutFull/{}.png", 1..=16, "Wallnut")?;


        // Load plant cards
        let peashooter_card = Image::new(ctx, "/plants/Peashooter.png")?;
        let sunflower_card = Image::new(ctx, "/plants/SunFlower.png")?;
        let wallnut_card = Image::new(ctx, "/plants/WallNut.png")?;


        // 加载僵尸图像
        let zombies_walk1_images = load_animation_frames(ctx, "/zombies/ZombieWalk1/{}.png", 1..=22, "Zombie walk")?;
        let zombie_attack_images = load_animation_frames(ctx, "/zombies/ZombieAttack/{}.png", 1..=21, "Zombie attack")?;
        let zombie_die_images = load_animation_frames(ctx, "/zombies/ZombieDie/{}.png", 1..=10, "Zombie die")?;
        let zombie_head_images = load_animation_frames(ctx, "/zombies/ZombieHead/{}.png", 1..=12, "Zombie head fall")?;

        // 加载豌豆图像
        let pea_image = Image::new(ctx, "/plants/Pea.png")?;
        // let pea_snow_image = Image::new(ctx, "/plants/PeaSnow.png")?;


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
            zombies_walk1_images,
            zombie_attack_images,
            zombie_die_images,
            zombie_head_images,
            pea_image,
            // pea_snow_image,
            // Assign other potentially unloaded Vecs as empty or handle appropriately
        })
    }
}
