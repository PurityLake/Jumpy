use bevy::prelude::*;

mod game;

use game::{game_over, main_game};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jumpy".into(),
                resolution: (300., 500.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((main_game::GamePlugin, game_over::GameOverPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
