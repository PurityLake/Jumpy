use bevy::prelude::*;

const MAX_Y_VEL: f32 = 100.;

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
                title: "Doodle Jump Clone".into(),
                resolution: (300., 500.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((game::GamePlugin, game_over::GameOverPlugin))
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

mod game {
    use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

    use super::{despawn_screen, GameState, MAX_Y_VEL};

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
}

mod game_over {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

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
}
