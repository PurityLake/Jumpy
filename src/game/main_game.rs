use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{despawn_screen, GameState};

const MAX_Y_VEL: f32 = 100.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, update.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Player::default(),
        OnGameScreen,
    ));
}

fn update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = sprite_position.single_mut();

    let mut translate = Vec3::ZERO;

    if transform.translation.y < -250. {
        game_state.set(GameState::GameOver);
    }

    if keyboard_input.pressed(KeyCode::Left) {
        translate.x = -100.;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        translate.x = 100.;
    }

    if player.falling {
        player.y_vel -= 10.;
        if player.y_vel < -MAX_Y_VEL {
            player.y_vel = -MAX_Y_VEL;
        }
    }

    translate.y = player.y_vel;

    transform.translation += translate * time.delta_seconds();
}

#[derive(Component)]
struct Player {
    falling: bool,
    y_vel: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            falling: true,
            y_vel: 0.,
        }
    }
}
