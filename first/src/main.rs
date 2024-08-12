use bevy::prelude::*;
use hello::HelloPlugin;

mod components;
mod hello;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
