use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use rand::prelude::*;
use std::time::Duration;

pub fn sun_collection_system(
    mut commands: Commands,
    mut sun_query: Query<(Entity, &Transform, &Sun)>,
    windows: Query<&Window>,
    // mouse_button_input: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut sun_counter: ResMut<SunCounter>,
) {
    // if mouse_button_input.just_pressed(MouseButton::Left) {
    //     let (camera, camera_transform) = camera_q.single();
    //     let window = windows.single();
        
        // if let Some(cursor_position) = window.cursor_position()
        //     .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        // {
        //     for (sun_entity, transform, sun) in &sun_query {
        //         let distance = transform.translation.truncate().distance(cursor_position);
        //         if distance < 30.0 {
        //             sun_counter.value += sun.value;
        //             commands.entity(sun_entity).despawn();
        //         }
        //     }
        // }
    // }
}
