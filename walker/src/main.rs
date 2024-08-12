use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*, render::camera::ScalingMode};
use bevy_turborand::prelude::RngPlugin;

mod walker;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.2, 0.4, 0.6)));
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Walker".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        FpsOverlayPlugin::default(),
        RngPlugin::default(),
    ));
    app.add_systems(Startup, spawn_camera);
    app.add_plugins(walker::WalkerPlugin);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(100.);
    commands.spawn(camera);
}
