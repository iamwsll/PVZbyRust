//! # 资源管理模块
//!
//! `resources` 模块负责加载和管理游戏所需的各种资源，如图片、字体等。
//! 它提供了一个统一的接口来访问这些资源，简化了游戏其他部分的资源处理逻辑。

use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::path::Path;

/// 存储所有游戏资源，如图像和字体。
///
/// 这个结构体在游戏启动时被实例化，并加载所有必要的视觉资源。
/// 之后，游戏的其他部分可以通过这个结构体来访问这些资源。
pub struct Resources {
    // 背景和UI元素
    /// 游戏主背景图片。
    pub background: Image,
    /// 商店界面的背景图片。
    pub shop_image: Image,
    /// 暂停按钮图片。
    pub button_image: Image,
    /// 太阳的动画帧序列。
    pub sun_images: Vec<Image>,
    /// 铲子图片。
    pub shovel_image: Image,
    /// 铲子框图片。
    pub shovel_bank_image: Image,

    // 植物相关图像
    /// 豌豆射手的动画帧序列。
    pub peashooter_images: Vec<Image>,
    /// 向日葵的动画帧序列。
    pub sunflower_images: Vec<Image>,
    /// 坚果墙的动画帧序列。
    pub wallnut_images: Vec<Image>,
    /// 商店中豌豆射手的卡片图像。
    pub peashooter_card: Image,
    /// 商店中向日葵的卡片图像。
    pub sunflower_card: Image,
    /// 商店中坚果墙的卡片图像。
    pub wallnut_card: Image,

    // 僵尸相关图像
    /// 普通僵尸行走动画帧序列。
    pub zombies_walk1_images: Vec<Image>,
    /// 普通僵尸攻击动画帧序列。
    pub zombie_attack_images: Vec<Image>,
    /// 普通僵尸死亡动画帧序列。
    pub zombie_die_images: Vec<Image>,
    /// 普通僵尸头部掉落动画帧序列。
    pub zombie_head_images: Vec<Image>,
    // 路障僵尸图像
    /// 路障僵尸行走动画帧序列。
    pub cone_zombie_walk_images: Vec<Image>,
    /// 路障僵尸攻击动画帧序列。
    pub cone_zombie_attack_images: Vec<Image>,

    // 豌豆相关图像
    /// 普通豌豆的图像。
    pub pea_image: Image,
    // /// 寒冰豌豆的图像。 (暂未使用)
    // pub pea_snow_image: Image,

    // 字体 (可选, 如果需要自定义文本渲染)
    // pub font: Font,
}

/// 加载一系列动画帧的辅助函数。
///
/// # Arguments
///
/// * `ctx` - ggez的上下文环境。
/// * `path_pattern` - 图像文件的路径模式，其中 `{}` 会被帧号替换。
/// * `frame_count` - 一个表示帧号范围的迭代器 (例如 `1..=10`)。
/// * `asset_name` - 资源的名称，用于在加载失败时打印警告信息。
///
/// # Returns
///
/// 返回一个 `GameResult`，其中包含一个 `Vec<Image>` (加载的动画帧) 或者一个错误。
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
    /// 创建并初始化 `Resources` 结构体，加载所有游戏资源。
    ///
    /// # Arguments
    ///
    /// * `ctx` - ggez的上下文环境。
    ///
    /// # Returns
    ///
    /// 返回一个 `GameResult`，其中包含初始化完成的 `Resources` 实例或者一个错误。
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        // Load background and UI
        let background = Image::new(ctx, "/other_image/Background.png")?;
        let shop_image = Image::new(ctx, "/other_image/Shop.png")?;
        let button_image = Image::new(ctx, "/other_image/Button.png")?;

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

        // 加载路障僵尸图像
        let cone_zombie_walk_images = load_animation_frames(ctx, "/zombies/ConeZombieWalk/{}.png", 1..=21, "Cone Zombie walk")?;
        let cone_zombie_attack_images = load_animation_frames(ctx, "/zombies/ConeZombieAttack/{}.png", 1..=11, "Cone Zombie attack")?;

        // 加载豌豆图像
        let pea_image = Image::new(ctx, "/plants/Pea.png")?;
        // let pea_snow_image = Image::new(ctx, "/plants/PeaSnow.png")?;

        // 加载铲子相关图像
        let shovel_image = Image::new(ctx, "/other_image/Shovel.png")?;
        let shovel_bank_image = Image::new(ctx, "/other_image/ShovelBank.png")?;

        Ok(Resources {
            background,
            shop_image,
            button_image,
            sun_images,
            shovel_image,
            shovel_bank_image,
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
            cone_zombie_walk_images,
            cone_zombie_attack_images,
            pea_image,
            // pea_snow_image,
            // Assign other potentially unloaded Vecs as empty or handle appropriately
        })
    }
}
