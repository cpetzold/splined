use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::PrimaryWindow,
};

use bevy_pancam::{PanCam, PanCamPlugin};

pub const MAIN_RENDER_LAYER: RenderLayers = RenderLayers::layer(0);
pub const GRID_RENDER_LAYER: RenderLayers = RenderLayers::layer(1);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_plugins(Material2dPlugin::<GridMaterial>::default())
            .add_systems(Startup, setup_cameras)
            .add_systems(PostUpdate, sync_grid_size);
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Grid;

fn setup_cameras(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Grid,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: Transform::from_xyz(0., 0., -1.),
                    material: materials.add(GridMaterial {}),
                    ..default()
                },
                MAIN_RENDER_LAYER,
            ));
        });

    // Grid camera
    // commands.spawn((
    //     GRID_RENDER_LAYER,
    //     Camera2dBundle {
    //         camera: Camera {
    //             order: 0,
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));

    // // Grid mesh
    // commands.spawn((
    //     Grid,
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(Rectangle::default()).into(),
    //         transform: Transform::from_xyz(0., 0., -10.),
    //         material: materials.add(GridMaterial {}),
    //         ..default()
    //     },
    //     GRID_RENDER_LAYER,
    // ));
}

// fn resize_grid_on_window_resize(
//     mut resize_events: EventReader<WindowResized>,
//     mut grid_query: Query<&mut Transform, With<Grid>>,
//     window_query: Query<Entity, With<PrimaryWindow>>,
// ) {
//     for &WindowResized {
//         window,
//         width,
//         height,
//     } in resize_events.read()
//     {
//         let Ok(primary_window) = window_query.get_single() else {
//             return;
//         };

//         if window != primary_window {
//             continue;
//         }

//         let Ok(mut grid_transform) = grid_query.get_single_mut() else {
//             return;
//         };

//         println!("Resizing grid to {}x{}", width, height);

//         grid_transform.scale = Vec3::new(width, height, 1.);
//     }
// }

fn sync_grid_size(
    mut grid_query: Query<&mut Transform, With<Grid>>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut grid_transform = grid_query.single_mut();
    let camera_projection = camera_query.single();
    let window = window_query.single();

    grid_transform.scale = Vec3::new(
        window.width() * camera_projection.scale,
        window.height() * camera_projection.scale,
        1.,
    );
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GridMaterial {}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}
