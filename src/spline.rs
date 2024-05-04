use bevy::{math::vec2, prelude::*};
use bevy_vello::vello::kurbo::{BezPath, Point};

pub struct BezierHandle {
    pub join: Vec2,
    pub c1: Vec2,
    pub c2: Vec2,
}

impl BezierHandle {
    pub fn new<P>(join: P, c1: P, c2: P) -> Self
    where
        P: Into<Vec2>,
    {
        BezierHandle {
            join: join.into(),
            c1: c1.into(),
            c2: c2.into(),
        }
    }

    pub fn from_join_and_tangent<P>(join: P, tangent: P) -> Self
    where
        P: Into<Vec2>,
    {
        let join = join.into();
        let tangent = tangent.into();
        let c1 = join + (join - tangent);
        let c2 = join + (tangent - join);
        BezierHandle { join, c1, c2 }
    }
}

#[derive(Component)]
pub struct TerrainSpline {
    pub handles: Vec<BezierHandle>,
}

impl TerrainSpline {
    pub fn new(handles: Vec<BezierHandle>) -> Self {
        assert_eq!(handles.len(), 3, "Terrain spline must have 3 handles");
        TerrainSpline { handles }
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

impl From<&TerrainSpline> for BezPath {
    fn from(spline: &TerrainSpline) -> Self {
        let mut path = BezPath::new();
        let first = spline.handles.first().unwrap();

        path.move_to(SplinePoint::from(first.join));
        for w in spline.handles.windows(2) {
            match w {
                [curr, next] => {
                    path.curve_to(
                        SplinePoint::from(curr.c2),
                        SplinePoint::from(next.c1),
                        SplinePoint::from(next.join),
                    );
                }
                _ => unreachable!(),
            }
        }
        let last = spline.handles.last().unwrap();
        path.curve_to(
            SplinePoint::from(last.c2),
            SplinePoint::from(first.c1),
            SplinePoint::from(first.join),
        );
        path.close_path();
        path
    }
}
