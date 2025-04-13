use bevy::prelude::*;
use bevy::{DefaultPlugins, app::App};

#[derive(States, Debug, Eq, PartialEq, Hash, Clone, Default)]
pub enum AppStates {
    Run,
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppStates>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppStates::Run), game::setup_game)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

mod game {
    use bevy::ecs::system::Commands;

    pub fn setup_game(mut commands: Commands) {
        // TODO;
    }
}
