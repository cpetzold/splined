mod camera;
mod editor;
mod spline;

use bevy::{
    math::vec2,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use bevy_vello::VelloPlugin;
use camera::CameraPlugin;
use editor::{EditorPlugin, Selected};
use spline::{
    Spline, SplineBundle, SplineControlPointBundle, SplineHandle, SplineHandleBundle, SplinePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Splined".into(),
                name: Some("splined.app".into()),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VelloPlugin)
        .add_plugins((CameraPlugin, EditorPlugin, SplinePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let handle1 = commands.spawn_empty().id();
    let c1a = commands
        .spawn((
            SplineControlPointBundle::new(vec2(-200., 300.), handle1),
            Selected,
        ))
        .id();
    let c1b = commands
        .spawn((
            SplineControlPointBundle::new(vec2(-200., -300.), handle1),
            Selected,
        ))
        .id();
    commands.entity(handle1).insert((
        SplineHandleBundle::new(
            SplineHandle {
                control_point_a: c1a,
                control_point_b: c1b,
            },
            vec2(-200., 0.),
        ),
        Selected,
    ));

    let handle2 = commands.spawn_empty().id();
    let c2a = commands
        .spawn((
            SplineControlPointBundle::new(vec2(200., -300.), handle2),
            Selected,
        ))
        .id();
    let c2b = commands
        .spawn((
            SplineControlPointBundle::new(vec2(200., 300.), handle2),
            Selected,
        ))
        .id();
    commands.entity(handle2).insert((
        SplineHandleBundle::new(
            SplineHandle {
                control_point_a: c2a,
                control_point_b: c2b,
            },
            vec2(200., 0.),
        ),
        Selected,
    ));

    commands.spawn((SplineBundle {
        spline: Spline {
            handles: vec![handle1, handle2],
        },
        ..default()
    },));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GridMaterial {}

impl Material for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}
