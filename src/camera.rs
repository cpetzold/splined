use bevy::{prelude::*, render::view::RenderLayers, sprite::MaterialMesh2dBundle};

use bevy_pancam::{PanCam, PanCamPlugin};

pub const MAIN_RENDER_LAYER: RenderLayers = RenderLayers::layer(0);
pub const GRID_RENDER_LAYER: RenderLayers = RenderLayers::layer(1);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, setup_cameras);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_cameras(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MainCamera,
        MAIN_RENDER_LAYER,
        Camera2dBundle {
            camera: Camera {
                order: 10,
                ..Default::default()
            },
            ..default()
        },
        PanCam::default(),
    ));

    // Grid camera
    commands.spawn((
        GRID_RENDER_LAYER,
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
    ));

    // Grid mesh
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_xyz(0., 0., -10.).with_scale(Vec3::new(1920., 1080., 1.)),
            material: materials.add(Color::PURPLE),
            ..default()
        },
        GRID_RENDER_LAYER,
    ));
}
