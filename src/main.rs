pub mod events;
mod systems;
mod game;
mod mainmenu;

use game::GamePlugin;
use mainmenu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plguins
        .add_plugins(DefaultPlugins)
        // Cutom Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        // 
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

