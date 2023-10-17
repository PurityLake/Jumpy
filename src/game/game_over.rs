use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), gameover_setup)
            .add_systems(
                Update,
                gameover_update.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_screen::<OnGameOverScreen>,
            );
    }
}

#[derive(Component)]
struct OnGameOverScreen;

fn gameover_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Game Over", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        OnGameOverScreen,
    ));
}

fn gameover_update(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game);
    }
}
