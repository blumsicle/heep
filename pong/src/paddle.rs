use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{ball::Ball, gutter::Gutter, Position, Reference, Shape, Velocity};

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(position: Position, shape: Shape) -> Self {
        Self {
            paddle: Paddle,
            shape,
            position,
            velocity: Velocity(Vec2::ZERO),
        }
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_paddles);
        app.add_systems(
            Update,
            (
                Self::move_paddles,
                Self::handle_ai.after(Self::move_paddles),
                Self::handle_player_input.after(Self::move_paddles),
            ),
        );
    }
}

impl PaddlePlugin {
    const SPEED: f32 = 2.;
    const WIDTH: f32 = 10.;
    const HEIGHT: f32 = 50.;

    fn spawn_paddles(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Query<&Window>,
    ) {
        debug!("spawining paddles");

        if let Ok(window) = window.get_single() {
            let window_width = window.resolution.width();
            let padding = 50.;
            let right_paddle_x = window_width / 2. - padding;
            let left_paddle_x = -window_width / 2. + padding;

            let mesh = Mesh::from(Rectangle::new(Self::WIDTH, Self::HEIGHT));
            let mesh_handle = meshes.add(mesh);

            commands.spawn((
                Player,
                PaddleBundle::new(
                    Position(Vec2::new(right_paddle_x, 0.)),
                    Shape(Vec2::new(Self::WIDTH, Self::HEIGHT)),
                ),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: materials.add(ColorMaterial::from(Color::srgb(0., 1., 0.))),
                    ..Default::default()
                },
            ));

            commands.spawn((
                Ai,
                PaddleBundle::new(
                    Position(Vec2::new(left_paddle_x, 0.)),
                    Shape(Vec2::new(Self::WIDTH, Self::HEIGHT)),
                ),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: materials.add(ColorMaterial::from(Color::srgb(0., 0., 1.))),
                    ..Default::default()
                },
            ));
        }
    }

    fn handle_player_input(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut paddle: Query<&mut Velocity, With<Player>>,
    ) {
        if let Ok(mut velocity) = paddle.get_single_mut() {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                velocity.0.y = 1.;
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                velocity.0.y = -1.;
            } else {
                velocity.0.y = 0.;
            }
        }
    }

    fn handle_ai(
        mut paddle: Query<(&mut Velocity, &Position), With<Ai>>,
        ball: Query<&Position, With<Ball>>,
    ) {
        if let (Ok((mut velocity, position)), Ok(ball_position)) =
            (paddle.get_single_mut(), ball.get_single())
        {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum() * 0.9;
        }
    }

    fn move_paddles(
        mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>,
        gutter_shape: Query<&Shape, (With<Gutter>, With<Reference>)>,
        window: Query<&Window>,
    ) {
        if let (Ok(window), Ok(gutter)) = (window.get_single(), gutter_shape.get_single()) {
            let window_height = window.resolution.height();
            let max_y = window_height / 2. - gutter.0.y - Self::HEIGHT / 2.;

            for (mut position, velocity) in &mut paddles {
                let new_position = position.0 + velocity.0 * Self::SPEED;
                if new_position.y.abs() < max_y {
                    position.0 = new_position;
                }
            }
        }
    }
}
