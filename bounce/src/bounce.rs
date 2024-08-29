use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use heep::{Position, Shape, Velocity};

#[derive(Component, Default)]
struct Ball;

#[derive(Bundle, Default)]
struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
        app.add_systems(Update, (Self::add_velocity, Self::check_bounds).chain());
    }
}

impl BallPlugin {
    const COLOR: Color = Color::srgba(0.2, 0.2, 0.8, 0.6);
    const RADIUS: f32 = 2.5;

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let shape = meshes.add(Circle::new(Self::RADIUS));
        let color = materials.add(Self::COLOR);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape.into(),
                material: color,
                ..Default::default()
            },
            BallBundle {
                velocity: Velocity(Vec2::new(10., 15.)),
                shape: Shape(Vec2::splat(Self::RADIUS)),
                ..Default::default()
            },
        ));
    }

    fn add_velocity(time: Res<Time>, mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
        if let Ok((mut position, velocity)) = ball.get_single_mut() {
            position.0 += velocity.0 * time.delta_seconds();
        }
    }

    fn check_bounds(
        window: Query<&Window>,
        camera: Query<(&Camera, &GlobalTransform)>,
        mut ball: Query<(&mut Position, &mut Velocity, &Shape), With<Ball>>,
    ) {
        let window = window.single();
        let (camera, camera_transform) = camera.single();
        let (mut position, mut velocity, shape) = ball.single_mut();

        let half_window_size = camera
            .viewport_to_world_2d(camera_transform, window.resolution.size())
            .unwrap()
            .abs();

        if position.0.x - shape.0.x < -half_window_size.x {
            position.0.x = -half_window_size.x + shape.0.x;
            velocity.0.x *= -1.;
        } else if position.0.x + shape.0.x > half_window_size.x {
            position.0.x = half_window_size.x - shape.0.x;
            velocity.0.x *= -1.;
        }

        if position.0.y - shape.0.y < -half_window_size.y {
            position.0.y = -half_window_size.y + shape.0.y;
            velocity.0.y *= -1.;
        } else if position.0.y + shape.0.y > half_window_size.y {
            position.0.y = half_window_size.y - shape.0.y;
            velocity.0.y *= -1.;
        }
    }
}
