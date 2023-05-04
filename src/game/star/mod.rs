use bevy::prelude::*;

pub mod components;
mod resources;
pub mod system_consts;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StarSpawnTimer>()
        //.add_startup_system(spawn_stars)
        // On Enter State
        .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
        // .add_system(tick_star_spawn_timer)
        // .add_system(spawn_stars_over_time);
        .add_systems(
            (
            tick_star_spawn_timer,
            spawn_stars_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // On Exit State
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
        ;
    }
}