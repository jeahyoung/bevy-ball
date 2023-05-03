use bevy::prelude::*;

pub mod components;
mod resources;
pub mod system_consts;
mod systems;

use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_stars)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time);
    }
}