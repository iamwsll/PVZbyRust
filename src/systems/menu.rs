use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::states::GameState;

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 主菜单UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                // 更新背景颜色设置方式
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "植物大战僵尸",
                        TextStyle {
                            // 字体加载应该使用通用系统字体或处理可能的加载失败
                            font_size: 64.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                }
            );

            // 开始按钮
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        // 更新背景颜色设置方式
                        background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "开始游戏",
                        TextStyle {
                            // 字体加载应该使用通用系统字体或处理可能的加载失败
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

pub fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        // 使用正确的比较方法
        if matches!(*interaction, Interaction::Pressed) {
            next_state.set(GameState::InGame);
        }
    }
}

pub fn load_game_textures(
    mut game_textures: ResMut<GameTextures>,
    asset_server: Res<AssetServer>,
) {
    // 加载植物贴图
    game_textures.plants.insert(PlantType::Peashooter, asset_server.load("images/plants/peashooter.png"));
    game_textures.plants.insert(PlantType::Sunflower, asset_server.load("images/plants/sunflower.png"));
    game_textures.plants.insert(PlantType::WallNut, asset_server.load("images/plants/wallnut.png"));
    game_textures.plants.insert(PlantType::CherryBomb, asset_server.load("images/plants/cherrybomb.png"));
    
    // 加载僵尸贴图
    game_textures.zombies.insert(ZombieType::Regular, asset_server.load("images/zombies/regular.png"));
    game_textures.zombies.insert(ZombieType::ConeHead, asset_server.load("images/zombies/conehead.png"));
    game_textures.zombies.insert(ZombieType::BucketHead, asset_server.load("images/zombies/buckethead.png"));
    
    // 加载其他贴图
    game_textures.backgrounds.insert("lawn".to_string(), asset_server.load("images/backgrounds/lawn.png"));
    game_textures.sun = asset_server.load("images/sun.png");
    game_textures.projectile = asset_server.load("images/projectile.png");
    
    // 加载卡片贴图
    game_textures.cards.insert(PlantType::Peashooter, asset_server.load("images/cards/peashooter_card.png"));
    game_textures.cards.insert(PlantType::Sunflower, asset_server.load("images/cards/sunflower_card.png"));
    game_textures.cards.insert(PlantType::WallNut, asset_server.load("images/cards/wallnut_card.png"));
    game_textures.cards.insert(PlantType::CherryBomb, asset_server.load("images/cards/cherrybomb_card.png"));
}
