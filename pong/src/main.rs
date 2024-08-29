use ball::BallPlugin;
use bevy::{log::LogPlugin, prelude::*};
use gutter::GutterPlugin;
use paddle::PaddlePlugin;
use scorer::ScorePlugin;

mod ball;
mod gutter;
mod paddle;
mod scorer;

#[derive(Component)]
pub struct Position(Vec2);

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component, Clone, Copy)]
pub struct Shape(Vec2);

#[derive(Component)]
pub struct Reference;

#[derive(Debug)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn main() {
    let mut app = App::new();
    #[cfg(debug_assertions)]
    let log = LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    };

    #[cfg(not(debug_assertions))]
    let log = LogPlugin {
        level: bevy::log::Level::INFO,
        ..Default::default()
    };

    app.add_plugins(DefaultPlugins.set(log));
    app.add_systems(Startup, spawn_camera);
    app.add_plugins(BallPlugin);
    app.add_plugins(GutterPlugin);
    app.add_plugins(PaddlePlugin);
    app.add_plugins(ScorePlugin);
    app.add_systems(PostUpdate, project_positions);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    debug!("spawning camera");
    commands.spawn(Camera2dBundle::default());
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
