mod spline;

use bevy::{math::vec2, prelude::*};
use bevy_editor_pls::{default_windows::cameras::EditorCamera, prelude::*};
use bevy_vello::VelloPlugin;
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
        .add_plugins(EditorPlugin::default())
        .add_plugins(VelloPlugin)
        .add_plugins(SplinePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_terrain)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let c1a = commands
        .spawn(SplineControlPointBundle::new(vec2(-300., 300.)))
        .id();
    let c1b = commands
        .spawn(SplineControlPointBundle::new(vec2(300., -300.)))
        .id();
    let handle1 = commands
        .spawn(SplineHandleBundle::new(SplineHandle {
            control_point_a: c1a,
            control_point_b: c1b,
        }))
        .push_children(&[c1a, c1b])
        .id();

    let c2a = commands
        .spawn(SplineControlPointBundle::new(vec2(-300., -300.)))
        .id();
    let c2b = commands
        .spawn(SplineControlPointBundle::new(vec2(300., 300.)))
        .id();
    let handle2 = commands
        .spawn(SplineHandleBundle::new(SplineHandle {
            control_point_a: c2a,
            control_point_b: c2b,
        }))
        .push_children(&[c2a, c2b])
        .id();

    commands
        .spawn(SplineBundle {
            spline: Spline {
                handles: vec![handle1, handle2],
            },
            ..default()
        })
        .push_children(&[handle1, handle2]);
}

fn update_terrain(
    mut handles: Query<&mut Transform, With<SplineHandle>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), Without<EditorCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    let Some(mut handle_transform) = handles.iter_mut().next() else {
        return;
    };

    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world(camera_transform, p))
        .map(|ray| ray.origin.truncate())
    {
        handle_transform.translation.x = pos.x;
        handle_transform.translation.y = pos.y;
    }
}
