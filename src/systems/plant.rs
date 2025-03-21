use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use std::time::Duration;

pub fn setup_game(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut game_grid: ResMut<GameGrid>,
) {
    // 创建游戏背景
    commands.spawn(SpriteBundle {
        texture: game_textures.backgrounds.get("lawn").unwrap().clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    
    // 初始化游戏网格
    commands.init_resource::<GameGrid>();
    
    // 创建植物选择器UI
    setup_plant_selector(&mut commands, &game_textures);
}

fn setup_plant_selector(commands: &mut Commands, game_textures: &Res<GameTextures>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    width: Val::Px(420.0),
                    height: Val::Px(90.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                // 更新背景颜色设置方式
                background_color: BackgroundColor(Color::rgba(0.3, 0.3, 0.3, 0.7)),
                ..default()
            },
            PlantSelector,
        ))
        .with_children(|parent| {
            // 创建植物卡片
            let plant_types = [
                PlantType::Sunflower,
                PlantType::Peashooter, 
                PlantType::WallNut, 
                PlantType::CherryBomb
            ];
            
            for plant_type in plant_types.iter() {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(70.0),
                                height: Val::Px(90.0),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            // 更新背景颜色设置方式
                            background_color: BackgroundColor(Color::WHITE),
                            ..default()
                        },
                        Card {
                            plant_type: *plant_type,
                            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            image: UiImage::new(game_textures.cards.get(plant_type).unwrap().clone()),
                            ..default()
                        });
                        
                        // 显示植物价格
                        parent.spawn(
                            TextBundle::from_section(
                                plant_type.cost().to_string(),
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                right: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                                ..default()
                            }),
                        );
                    });
            }
        });
}

pub fn plant_placement_system(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    game_textures: Res<GameTextures>,
    mut sun_counter: ResMut<SunCounter>,
    mut game_grid: ResMut<GameGrid>,
    // 其他需要的资源和组件查询
) {
    // 如果点击了左键
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // 获取光标位置
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();
        
        if let Some(cursor_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            // 计算网格位置
            let grid_x = ((cursor_position.x + 400.0) / 80.0) as usize;
            let grid_y = ((300.0 - cursor_position.y) / 100.0) as usize;
            
            // 检查点击是否在有效网格内
            if grid_x < 9 && grid_y < 5 {
                // 检查网格是否为空
                if game_grid.grid[grid_x][grid_y].is_none() {
                    // 植物放置逻辑（这里简化为只放置豌豆射手）
                    let plant_type = PlantType::Peashooter;
                    let cost = plant_type.cost();
                    
                    // 检查是否有足够的阳光
                    if sun_counter.value >= cost {
                        // 消耗阳光
                        sun_counter.value -= cost;
                        
                        // 创建植物实体
                        let plant_entity = commands.spawn((
                            SpriteBundle {
                                texture: game_textures.plants.get(&plant_type).unwrap().clone(),
                                transform: Transform::from_xyz(
                                    grid_x as f32 * 80.0 - 360.0, 
                                    250.0 - grid_y as f32 * 100.0, 
                                    1.0
                                ),
                                ..default()
                            },
                            Plant {
                                plant_type,
                                health: 100.0,
                                cost,
                                cooldown: Timer::from_seconds(1.5, TimerMode::Repeating),
                            },
                            GridPosition {
                                x: grid_x,
                                y: grid_y,
                            },
                        )).id();
                        
                        // 更新网格
                        game_grid.grid[grid_x][grid_y] = Some(plant_entity);
                    }
                }
            }
        }
    }
}

pub fn plant_shooting_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Plant, &Transform, &GridPosition)>,
    zombie_query: Query<(&Zombie, &Transform, &GridPosition)>,
    game_textures: Res<GameTextures>,
) {
    for (mut plant, plant_transform, plant_grid) in &mut query {
        // 只有豌豆射手才射击
        if plant.plant_type == PlantType::Peashooter {
            // 更新冷却时间
            plant.cooldown.tick(time.delta());
            
            // 检查是否可以射击
            if plant.cooldown.just_finished() {
                // 检查同一行是否有僵尸
                let mut has_zombie_in_lane = false;
                
                for (zombie, zombie_transform, zombie_grid) in &zombie_query {
                    if zombie_grid.y == plant_grid.y && zombie_grid.x > plant_grid.x {
                        has_zombie_in_lane = true;
                        break;
                    }
                }
                
                // 如果有僵尸，发射豌豆
                if has_zombie_in_lane {
                    commands.spawn((
                        SpriteBundle {
                            texture: game_textures.projectile.clone(),
                            transform: Transform::from_translation(
                                plant_transform.translation + Vec3::new(30.0, 0.0, 0.0)
                            ),
                            ..default()
                        },
                        Projectile {
                            damage: 20.0,
                            speed: 300.0,
                        },
                    ));
                }
            }
        }
    }
}

// 添加游戏结束设置函数
pub fn setup_game_over(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    // 创建游戏结束界面
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // 更新背景颜色设置方式
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
        ..default()
    })
    .with_children(|parent| {
        // 游戏结束文本
        parent.spawn(TextBundle::from_section(
            "游戏结束!",
            TextStyle {
                font_size: 80.0,
                color: Color::RED,
                ..default()
            },
        ));
    });
}
