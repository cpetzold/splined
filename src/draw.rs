use bevy::{ecs::system::SystemParam, prelude::*, render::color::Color};
use bevy_vello::{
    prelude::*,
    vello::{
        kurbo::{Affine, Point, Shape, Stroke},
        peniko::{BrushRef, Fill},
    },
    VelloPlugin,
};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloPlugin)
            .add_systems(Startup, setup_vector_graphics)
            .add_systems(PreUpdate, clear_scene);
    }
}

fn setup_vector_graphics(mut commands: Commands) {
    commands.spawn(VelloSceneBundle::default());
}

fn clear_scene(mut draw: Draw) {
    draw.reset();
}

#[derive(SystemParam)]
pub struct Draw<'w, 's> {
    scene: Query<'w, 's, &'static mut VelloScene>,
}

impl<'w, 's> Draw<'w, 's> {
    fn reset(&mut self) {
        let mut scene = self.scene.single_mut();
        scene.reset();
    }

    fn fill<'b>(
        &mut self,
        style: Fill,
        transform: Affine,
        brush: impl Into<BrushRef<'b>>,
        brush_transform: Option<Affine>,
        shape: &impl Shape,
    ) {
        let mut scene = self.scene.single_mut();
        scene.fill(style, transform, brush, brush_transform, shape);
    }

    pub fn fill_color(
        &mut self,
        style: Fill,
        color: impl Into<ColorWrapper>,
        transform: Affine,
        shape: &impl Shape,
    ) {
        let c = peniko::Color::from(color.into());
        self.fill(style, transform, BrushRef::Solid(c), None, shape);
    }

    fn stroke<'b>(
        &mut self,
        style: &Stroke,
        transform: Affine,
        brush: impl Into<BrushRef<'b>>,
        brush_transform: Option<Affine>,
        shape: &impl Shape,
    ) {
        let mut scene = self.scene.single_mut();
        scene.stroke(style, transform, brush, brush_transform, shape);
    }

    pub fn stroke_color(
        &mut self,
        style: &Stroke,
        color: impl Into<ColorWrapper>,
        transform: Affine,
        shape: &impl Shape,
    ) {
        let c = peniko::Color::from(color.into());
        self.stroke(style, transform, BrushRef::Solid(c), None, shape);
    }

    pub fn circle(&mut self, center: impl Into<Vec2>, radius: f32, color: impl Into<ColorWrapper>) {
        let center = center.into();
        let shape =
            kurbo::Circle::new(Point::new(center.x as f64, -center.y as f64), radius as f64);
        self.fill_color(Fill::NonZero, color, Affine::default(), &shape);
    }

    // pub fn stroke_bezier(&mut self, points: Vec<Vec2>, color: impl Into<ColorWrapper>) {
    //     let shape = kurbo::BezPath::from_vec(
    //         points
    //             .into_iter()
    //             .map(|p| Point::new(p.x as f64, -p.y as f64))
    //             .collect(),
    //     );
    //     self.stroke_color(&Stroke::new(1.0), color, Affine::default(), &shape);
    // }
}

pub struct ColorWrapper(Color);

impl From<ColorWrapper> for Color {
    fn from(wrapper: ColorWrapper) -> Self {
        wrapper.0
    }
}

impl From<Color> for ColorWrapper {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<ColorWrapper> for peniko::Color {
    fn from(wrapper: ColorWrapper) -> Self {
        Self::from(wrapper.0.as_rgba_u8())
    }
}

impl From<ColorWrapper> for BrushRef<'_> {
    fn from(wrapper: ColorWrapper) -> Self {
        BrushRef::Solid(wrapper.into())
    }
}
