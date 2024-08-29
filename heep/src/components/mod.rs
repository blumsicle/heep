use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Shape(pub Vec2);
