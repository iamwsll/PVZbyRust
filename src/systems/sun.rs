use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use rand::prelude::*;
use std::time::Duration;

pub fn sun_collection_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sun_query: Query<(Entity, &mut Transform, &mut Sun)>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut sun_counter: ResMut<SunCounter>,
) {
    // 生成随机阳光
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.005) {  // 每帧有0.5%的几率生成阳光
        let x = rng.gen_range(-350.0..350.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, 300.0, 2.0),
                texture: sun_counter.sun_texture.clone(), // 确保SunCounter有sun_texture字段
                ..default()
            },
            Sun {
                value: 25,
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            },
        ));
    }
    
    // 处理阳光
    for (sun_entity, mut transform, mut sun) in &mut sun_query {
        // 更新阳光下落
        if transform.translation.y > -200.0 {
            transform.translation.y -= 50.0 * time.delta_seconds();
        }
        
        // 更新阳光计时器
        sun.lifetime.tick(time.delta_seconds().into());
        if sun.lifetime.finished() {
            commands.entity(sun_entity).despawn();
            continue;
        }
        
        // 检查是否点击了阳光
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let (camera, camera_transform) = camera_q.single();
            let window = windows.single();
            
            if let Some(cursor_position) = window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                let distance = cursor_position.distance(transform.translation.truncate());
                if distance < 30.0 {
                    // 收集阳光
                    sun_counter.value += sun.value;
                    commands.entity(sun_entity).despawn();
                }
            }
        }
    }
    
    // 处理向日葵产生阳光
    let mut sunflower_query = Query<(&Plant, &Transform), With<Plant>>;
    for (plant, transform) in &sunflower_query {
        if plant.plant_type == PlantType::Sunflower {
            // 这里可以添加向日葵产生阳光的逻辑
        }
    }
}
