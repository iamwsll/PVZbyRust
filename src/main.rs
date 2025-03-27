use bevy::prelude::*;

mod plugin;
mod components;
mod resources;
mod systems;
mod states;

use plugin::PVZPlugin;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PVZPlugin)
        .add_state::<GameState>()
        .run();
}
