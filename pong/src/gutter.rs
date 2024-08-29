use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{Position, Reference, Shape};

#[derive(Component)]
pub struct Gutter;

#[derive(Bundle)]
struct GutterBundle {
    gutter: Gutter,
    position: Position,
    shape: Shape,
}

impl GutterBundle {
    fn new(position: Position, shape: Shape) -> Self {
        Self {
            gutter: Gutter,
            position,
            shape,
        }
    }
}

pub struct GutterPlugin;

impl Plugin for GutterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_gutters);
    }
}

impl GutterPlugin {
    const HEIGHT: f32 = 20.;

    fn spawn_gutters(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Query<&Window>,
    ) {
        if let Ok(window) = window.get_single() {
            let window_width = window.resolution.width();
            let window_height = window.resolution.height();

            let top_gutter_y = window_height / 2. - Self::HEIGHT / 2.;
            let bottom_gutter_y = -window_height / 2. + Self::HEIGHT / 2.;
            let gutter_shape = Shape(Vec2::new(window_width, Self::HEIGHT));

            let top_gutter =
                GutterBundle::new(Position(Vec2::new(0., top_gutter_y)), gutter_shape.clone());
            let bottom_gutter =
                GutterBundle::new(Position(Vec2::new(0., bottom_gutter_y)), gutter_shape);

            let mesh = Mesh::from(Rectangle::from_size(gutter_shape.0));
            let material = ColorMaterial::from(Color::srgb(0., 0., 0.));

            let mesh_handle = meshes.add(mesh);
            let material_handle = materials.add(material);

            commands.spawn((
                Reference,
                top_gutter,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: material_handle.clone(),
                    ..Default::default()
                },
            ));

            commands.spawn((
                bottom_gutter,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    ..Default::default()
                },
            ));
        }
    }
}
