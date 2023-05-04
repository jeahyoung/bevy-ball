use bevy::prelude::*;

pub mod components;
mod systems;


use systems::*;

use crate::AppState;

use super::SimulationState;


// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(
            (
                PlayerSystemSet::Movement,
                PlayerSystemSet::Confinement
            ).chain()
        )
        //.add_startup_system(spawn_player)
        // On Enter State
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_systems(
            (
                player_movement
                .in_set(PlayerSystemSet::Movement),
                confine_player_movement
                .in_set(PlayerSystemSet::Confinement)
                
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // .add_system(enemy_hit_player)
        // .add_system(player_hit_star)
        .add_systems(
            (
            enemy_hit_player,
            player_hit_star
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // On Exit State
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
        ;
        // .configure_sets(
        //     (
        //         MovementSystemSet,
        //         ConfinementSystemSet
        //     ).chain()
        // )
        // .add_startup_system(spawn_player)
        // // .add_system(player_movement)
        // // .add_system(confine_player_movement.after(player_movement))
        // .add_systems(
        //     (
        //         player_movement.in_set(MovementSystemSet),
        //         confine_player_movement.in_set(ConfinementSystemSet)
        //     ).chain()
        // )
        // .add_system(enemy_hit_player)
        // .add_system(player_hit_star);
    }
}