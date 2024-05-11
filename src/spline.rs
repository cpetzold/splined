use bevy::{math::vec2, prelude::*};
use bevy_vello::{
    vello::{
        kurbo::{Affine, BezPath, Circle, Line, Point, Stroke},
        peniko::{self, BrushRef, Fill},
    },
    CoordinateSpace, VelloScene, VelloSceneBundle,
};

use crate::editor::Selected;

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (render_splines, render_handles, render_control_points),
        );
    }
}

#[derive(Component)]
pub struct SplineControlPoint {
    pub handle: Entity,
}

#[derive(Bundle)]
pub struct SplineControlPointBundle {
    pub control_point: SplineControlPoint,
    pub scene_bundle: VelloSceneBundle,
}

impl SplineControlPointBundle {
    pub fn new(pos: Vec2, handle: Entity) -> Self {
        Self {
            control_point: SplineControlPoint { handle },
            scene_bundle: VelloSceneBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 5.),
                ..default()
            },
        }
    }
}

#[derive(Component, Default)]
pub enum ControlMode {
    #[default]
    Vector,
    Aligned,
    Free,
    Automatic,
}

#[derive(Component, Default)]
pub enum HandleControlMode {
    #[default]
    Inherit,
    Custom(ControlMode),
}

#[derive(Component)]
pub struct SplineHandle {
    pub control_point_a: Entity,
    pub control_point_b: Entity,
}

#[derive(Bundle)]
pub struct SplineHandleBundle {
    pub handle: SplineHandle,
    pub handle_control_mode: HandleControlMode,
    pub scene: VelloSceneBundle,
}

impl SplineHandleBundle {
    pub fn new(handle: SplineHandle, pos: Vec2) -> Self {
        Self {
            handle,
            handle_control_mode: HandleControlMode::default(),
            scene: VelloSceneBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 10.),
                ..default()
            },
        }
    }
}

#[derive(Component, Default)]
pub struct Spline {
    pub handles: Vec<Entity>,
}

#[derive(Bundle, Default)]
pub struct SplineBundle {
    pub spline: Spline,
    pub control_mode: ControlMode,
    pub spatial: SpatialBundle,
    pub scene: VelloScene,
    pub coordinate_space: CoordinateSpace,
}

fn render_splines(
    mut splines: Query<(&Spline, &mut VelloScene)>,
    handles: Query<(&GlobalTransform, &SplineHandle)>,
    control_points: Query<&GlobalTransform, With<SplineControlPoint>>,
) {
    for (spline, mut scene) in splines.iter_mut() {
        scene.reset();

        let handles = spline
            .handles
            .iter()
            .map(|&e| {
                let (handle_transform, handle) = handles.get(e).unwrap();
                let control_a = control_points
                    .get(handle.control_point_a)
                    .unwrap()
                    .translation()
                    .truncate();
                let control_b = control_points
                    .get(handle.control_point_b)
                    .unwrap()
                    .translation()
                    .truncate();
                (
                    handle_transform.translation().truncate(),
                    control_a,
                    control_b,
                )
            })
            .collect::<Vec<_>>();

        let mut path = BezPath::new();
        let first = handles.first().unwrap();
        path.move_to(SplinePoint::from(first.0));
        for w in handles.windows(2) {
            match w {
                [curr, next] => {
                    path.curve_to(
                        SplinePoint::from(curr.2),
                        SplinePoint::from(next.1),
                        SplinePoint::from(next.0),
                    );
                }
                _ => unreachable!(),
            }
        }
        let last = handles.last().unwrap();
        path.curve_to(
            SplinePoint::from(last.2),
            SplinePoint::from(first.1),
            SplinePoint::from(first.0),
        );

        path.close_path();

        scene.fill(
            Fill::EvenOdd,
            Affine::IDENTITY,
            peniko::Color::DARK_GRAY,
            None,
            &path,
        );

        scene.stroke(
            &Stroke::default(),
            Affine::IDENTITY,
            peniko::Color::WHITE,
            None,
            &path,
        );
    }
}

fn render_handles(mut handles: Query<(Option<&Selected>, &mut VelloScene), With<SplineHandle>>) {
    for (selected, mut scene) in handles.iter_mut() {
        let selected = selected.is_some();
        scene.reset();

        let circle = Circle::new(Point::ZERO, 4.0);

        scene.fill(
            Fill::EvenOdd,
            Affine::IDENTITY,
            if selected {
                peniko::Color::DARK_RED
            } else {
                peniko::Color::DARK_GRAY
            },
            None,
            &circle,
        );

        scene.stroke(
            &Stroke::new(1.0),
            Affine::IDENTITY,
            if selected {
                peniko::Color::RED
            } else {
                peniko::Color::WHITE
            },
            None,
            &circle,
        );
    }
}

fn render_control_points(
    mut control_points: Query<(
        &SplineControlPoint,
        &GlobalTransform,
        Option<&Selected>,
        &mut VelloScene,
    )>,
    handles: Query<&GlobalTransform, With<SplineHandle>>,
) {
    for (control_point, transform, selected, mut scene) in control_points.iter_mut() {
        let Ok(handle_transform) = handles.get(control_point.handle) else {
            continue;
        };

        let selected = selected.is_some();

        scene.reset();

        scene.stroke(
            &Stroke::new(0.5),
            Affine::IDENTITY,
            if selected {
                peniko::Color::RED
            } else {
                peniko::Color::WHITE
            },
            None,
            &Line::new(
                Point::ZERO,
                SplinePoint::from(
                    (-transform.translation() + handle_transform.translation()).truncate(),
                ),
            ),
        );

        scene.fill(
            Fill::EvenOdd,
            Affine::IDENTITY,
            if selected {
                peniko::Color::RED
            } else {
                peniko::Color::WHITE
            },
            None,
            &Circle::new(Point::ZERO, 2.0),
        );
    }
}

#[derive(Clone, Copy)]
struct SplinePoint(Vec2);

impl From<Vec2> for SplinePoint {
    fn from(point: Vec2) -> Self {
        SplinePoint(point)
    }
}

impl From<SplinePoint> for Vec2 {
    fn from(point: SplinePoint) -> Self {
        point.0
    }
}

impl From<SplinePoint> for Point {
    fn from(point: SplinePoint) -> Self {
        Point::new(point.0.x as f64, -point.0.y as f64)
    }
}

impl From<Point> for SplinePoint {
    fn from(point: Point) -> Self {
        SplinePoint(vec2(point.x as f32, -point.y as f32))
    }
}
