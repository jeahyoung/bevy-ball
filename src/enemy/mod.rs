use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod system_consts;
mod systems;

use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_startup_system(spawn_enemies)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_time);

    }
}