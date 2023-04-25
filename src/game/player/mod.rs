use bevy::prelude::*;

mod components;
mod systems;

use crate::AppState;
use crate::game::SimulationState;
use systems::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // Enter State Systems
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(player_movement
                .in_set(MovementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            .add_system(confine_player_movement
                .in_set(ConfinementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                (
                    enemy_hit_player,
                    player_hit_star
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            // Exit State Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
