// mod draw;
mod spline;

use bevy::{math::vec2, prelude::*};
// use draw::{Draw, DrawPlugin};
use bevy_vello::VelloPlugin;
use spline::{BezierHandle, TerrainSpline};

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
        .add_systems(Startup, setup)
        .add_systems(Update, update_terrain)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerrainSpline::new(vec![
        BezierHandle::from_join_and_tangent(vec2(-100., -100.), vec2(100., -100.)),
        BezierHandle::from_join_and_tangent(vec2(100., -100.), vec2(0., 100.)),
        BezierHandle::from_join_and_tangent(vec2(0., 100.), vec2(-100., -100.)),
    ]));

    commands.spawn(Camera2dBundle::default());
}

// fn render_terrain(terrain_query: Query<&TerrainSpline>, mut draw: Draw) {
//     for spline in &terrain_query {
//         let path = BezPath::from(spline);
//         draw.fill_color(Fill::EvenOdd, Color::GRAY, Affine::default(), &path);
//         draw.stroke_color(&Stroke::default(), Color::WHITE, Affine::default(), &path);

//         for handle in &spline.handles {
//             draw.circle(handle.join, 4., Color::WHITE);
//             draw.circle(handle.c1, 4., Color::RED);
//             draw.circle(handle.c2, 4., Color::RED);
//         }
//     }
// }

fn update_terrain(
    mut terrain_query: Query<&mut TerrainSpline>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    for mut spline in &mut terrain_query {
        if let Some(pos) = window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world(camera_transform, p))
            .map(|ray| ray.origin.truncate())
        {
            spline.handles[1].join = pos;
        }
    }
}
