use crate::{
    core_3d::graph::SubGraph3d,
    tonemapping::{DebandDither, Tonemapping},
};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::{
    camera::{
        Camera, CameraMainTextureUsages, CameraProjection, CameraRenderGraph,
        OrthographicProjection,
    },
    extract_component::ExtractComponent,
    primitives::Frustum,
    view::VisibleEntities,
};
use bevy_transform::prelude::{GlobalTransform, Transform};

use super::graph::SubGraph2d;

#[derive(Component, Default, Reflect, Clone, ExtractComponent)]
#[extract_component_filter(With<Camera>)]
#[reflect(Component)]
pub struct Camera2d;

#[derive(Bundle)]
pub struct Camera2dBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
    pub tonemapping: Tonemapping,
    pub deband_dither: DebandDither,
    pub main_texture_usages: CameraMainTextureUsages,
}

impl Default for Camera2dBundle {
    fn default() -> Self {
        let projection = OrthographicProjection {
            far: 1000.,
            near: -1000.,
            ..Default::default()
        };
        let transform = Transform::default();
        let frustum = projection.compute_frustum(&GlobalTransform::from(transform));
        Self {
            camera_render_graph: CameraRenderGraph::new(SubGraph2d),
            projection,
            visible_entities: VisibleEntities::default(),
            frustum,
            transform,
            global_transform: Default::default(),
            camera: Camera::default(),
            camera_2d: Camera2d,
            tonemapping: Tonemapping::None,
            deband_dither: DebandDither::Disabled,
            main_texture_usages: Default::default(),
        }
    }
}

impl Camera2dBundle {
    /// Create an orthographic projection camera with a custom `Z` position.
    ///
    /// The camera is placed at `Z=far-0.1`, looking toward the world origin `(0,0,0)`.
    /// Its orthographic projection extends from `0.0` to `-far` in camera view space,
    /// corresponding to `Z=far-0.1` (closest to camera) to `Z=-0.1` (furthest away from
    /// camera) in world space.
    pub fn new_with_far(far: f32) -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let projection = OrthographicProjection {
            far,
            ..Default::default()
        };
        let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
        let frustum = projection.compute_frustum(&GlobalTransform::from(transform));
        Self {
            camera_render_graph: CameraRenderGraph::new(SubGraph3d),
            projection,
            visible_entities: VisibleEntities::default(),
            frustum,
            transform,
            global_transform: Default::default(),
            camera: Camera::default(),
            camera_2d: Camera2d,
            tonemapping: Tonemapping::None,
            deband_dither: DebandDither::Disabled,
            main_texture_usages: Default::default(),
        }
    }
}
