use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_turborand::{DelegatedRng, RngComponent};

#[derive(Component)]
struct LastWalker;

#[derive(Resource)]
struct WalkerShape(Mesh2dHandle);

#[derive(Resource)]
struct WalkerColor(Handle<ColorMaterial>);

#[derive(Resource)]
struct SpawnTimer(Timer);

pub struct WalkerPlugin;

impl Plugin for WalkerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));
        app.add_systems(Startup, Self::setup);
        app.add_systems(Update, Self::update);
    }
}

impl WalkerPlugin {
    const COLOR: Color = Color::srgba(0.2, 0.2, 0.8, 0.6);
    const RADIUS: f32 = 1.5;

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let shape = Mesh2dHandle(meshes.add(Circle {
            radius: Self::RADIUS,
        }));
        let color = materials.add(Self::COLOR);

        commands.insert_resource(WalkerShape(shape.clone()));
        commands.insert_resource(WalkerColor(color.clone()));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: color,
                ..Default::default()
            },
            LastWalker,
        ));
    }

    fn update(
        mut commands: Commands,
        time: Res<Time>,
        mut timer: ResMut<SpawnTimer>,
        shape: Res<WalkerShape>,
        color: Res<WalkerColor>,
        query: Query<(Entity, &Transform), With<LastWalker>>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            let (entity, transform) = query.single();
            commands.entity(entity).remove::<LastWalker>();

            let mut rng = RngComponent::new();
            let mut transform = transform.clone();
            transform.translation.x += rng.i32(-1..=1) as f32 * Self::RADIUS * 2.;
            transform.translation.y += rng.i32(-1..=1) as f32 * Self::RADIUS * 2.;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: shape.0.clone(),
                    material: color.0.clone(),
                    transform,
                    ..Default::default()
                },
                LastWalker,
            ));
        }
    }
}
