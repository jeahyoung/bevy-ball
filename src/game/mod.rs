pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

use crate::{events::GameOver, AppState};

use self::systems::{toggle_simulation, pause_simulation, resume_simulation};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        // Events
        .add_event::<GameOver>()
        // On Enter Systems
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        // Plugins
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        // Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        // On Exit System
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
        ;
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,    
}