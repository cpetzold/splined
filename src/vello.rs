use bevy::{
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
    utils::synccell::SyncCell,
};
use std::{marker::PhantomData, num::NonZeroUsize};
use vello::{Renderer, RendererOptions, Scene};

pub struct VelloPlugin;

impl Plugin for VelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<VelloScene>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.add_systems(Render, render_scenes.in_set(RenderSet::Render));
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<VelloRenderer>();
    }
}

#[derive(Resource)]
struct VelloRenderer(SyncCell<Renderer>);

impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        VelloRenderer(SyncCell::new(
            Renderer::new(
                device.wgpu_device(),
                RendererOptions {
                    surface_format: None,
                    num_init_threads: NonZeroUsize::new(1),
                    antialiasing_support: vello::AaSupport::area_only(),
                    use_cpu: false,
                },
            )
            .unwrap(),
        ))
    }
}

fn render_scenes(
    mut renderer: ResMut<VelloRenderer>,
    mut scenes: Query<&VelloScene>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    for scene in &mut scenes {
        let gpu_image = gpu_images.get(&scene.image).unwrap();
        let params = vello::RenderParams {
            base_color: scene.background_color,
            width: gpu_image.size.x as u32,
            height: gpu_image.size.y as u32,
            antialiasing_method: vello::AaConfig::Area,
        };
        renderer
            .0
            .get()
            .render_to_texture(
                device.wgpu_device(),
                &queue,
                &scene.scene,
                &gpu_image.texture_view,
                &params,
            )
            .unwrap();
    }
}

#[derive(Component, Clone, ExtractComponent)]
pub struct VelloScene {
    pub scene: Scene,
    pub image: Handle<Image>,
    pub background_color: vello::peniko::Color,
}

impl From<Handle<Image>> for VelloScene {
    fn from(value: Handle<Image>) -> Self {
        Self {
            scene: Scene::default(),
            image: value,
            background_color: vello::peniko::Color::TRANSPARENT,
        }
    }
}

pub trait VelloRender {
    fn render(&self, scene: &mut Scene);
    fn size() -> (u32, u32);
    fn background_color(&self) -> vello::peniko::Color {
        vello::peniko::Color::TRANSPARENT
    }
}

pub struct VelloRenderPlugin<C: Component + VelloRender>(PhantomData<C>);

impl<C: Component + VelloRender> Default for VelloRenderPlugin<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C: Component + VelloRender> Plugin for VelloRenderPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (add_vello_scene::<C>, vello_render::<C>).chain(),
        );
    }
}

fn vello_render<C: Component + VelloRender>(
    mut components: Query<(&C, &mut VelloScene), Changed<C>>,
) {
    for (component, mut scene) in &mut components {
        scene.scene.reset();
        scene.background_color = component.background_color();
        component.render(&mut scene.scene);
    }
}

fn add_vello_scene<C: Component + VelloRender>(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    entities: Query<Entity, Added<C>>,
) {
    let (width, height) = C::size();
    for entity in &entities {
        let size = Extent3d {
            width,
            height,
            ..default()
        };

        // This is the texture that will be rendered to.
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::STORAGE_BINDING,
                view_formats: &[],
            },
            ..default()
        };

        // fill image.data with zeroes
        image.resize(size);

        let image_handle = images.add(image);
        let rectangle = meshes.add(Rectangle::new(width as f32, height as f32));

        // This material has the texture that has been rendered.
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(image_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands.entity(entity).insert((
            rectangle,
            material_handle,
            VelloScene {
                image: image_handle,
                scene: Scene::default(),
                background_color: vello::peniko::Color::TRANSPARENT,
            },
        ));
    }
}

#[derive(Bundle)]
pub struct VelloRenderBundle<C: Component + VelloRender> {
    pub render: C,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}

impl<C: Component + VelloRender + Default> Default for VelloRenderBundle<C> {
    fn default() -> Self {
        Self {
            render: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}
