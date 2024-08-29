use std::cmp::Ordering;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{Collision, Position, Shape, Velocity};

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl BallBundle {
    fn new(velocity: Velocity, shape: Shape) -> Self {
        Self {
            ball: Ball,
            shape,
            position: Position(Vec2::ZERO),
            velocity,
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_ball);
        app.add_systems(Update, (Self::handle_collisions, Self::move_ball).chain());
    }
}

impl BallPlugin {
    const SIZE: f32 = 5.;

    fn spawn_ball(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        debug!("spawning ball");

        let shape = Mesh::from(Circle::new(Self::SIZE));
        let color = ColorMaterial::from(Color::srgb(1., 0., 0.));

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            BallBundle::new(
                Velocity(Vec2::new(1., 2.)),
                Shape(Vec2::new(Self::SIZE, Self::SIZE)),
            ),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..Default::default()
            },
        ));
    }

    pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
        if let Ok((mut position, velocity)) = ball.get_single_mut() {
            position.0 += velocity.0;
        }
    }

    pub fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&wall) {
            return None;
        }

        let closest_point = wall.closest_point(ball.center());
        let offset = ball.center() - closest_point;

        let side = if offset.x.abs() > offset.y.abs() {
            match &offset.x.total_cmp(&0.) {
                Ordering::Less => Collision::Left,
                _ => Collision::Right,
            }
        } else {
            match &offset.y.total_cmp(&0.) {
                Ordering::Greater => Collision::Top,
                _ => Collision::Bottom,
            }
        };

        Some(side)
    }

    pub fn handle_collisions(
        mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
        others: Query<(&Position, &Shape), Without<Ball>>,
    ) {
        if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
            for (position, shape) in &others {
                if let Some(collision) = Self::collide_with_side(
                    BoundingCircle::new(ball_position.0, ball_shape.0.x),
                    Aabb2d::new(position.0, shape.0 / 2.),
                ) {
                    match collision {
                        Collision::Left | Collision::Right => ball_velocity.0.x *= -1.0,
                        Collision::Top | Collision::Bottom => ball_velocity.0.y *= -1.0,
                    }
                }
            }
        }
    }
}
