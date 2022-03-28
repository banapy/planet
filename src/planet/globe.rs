use bevy_utils::Uuid;
use bevy::prelude::{PerspectiveCameraBundle};
use bevy::prelude::*;
use std::default::{Default};
use crate::planet::constant::EarthConstant;

#[derive(Component)]
pub struct UniqueId {
    id: Uuid,
    label: String,
}

impl UniqueId {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: name,
        }
    }
}

#[derive(Component)]
pub struct Rotable {
    speed: f32,
}

impl Default for Rotable {
    fn default() -> Self {
        Self {
            speed: 0.0
        }
    }
}

#[derive(Component)]
pub struct Transform3d {
    ///local transform
    pub transform: Transform,
    ///local transform
    pub world_transform: Transform,
}

impl Default for Transform3d {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Bundle)]
pub struct GlobeBundle {
    pub name: UniqueId,
    pub transform: Transform3d,
    pub rotable: Rotable,
}

impl GlobeBundle {
    pub fn new() -> Self {
        let config: GlobeBundleConfig = Default::default();
        GlobeBundle::from_config(config)
    }
    pub fn from_config(config: GlobeBundleConfig) -> Self {
        GlobeBundle {
            name: UniqueId::new(config.name),
            transform: Default::default(),
            rotable: Default::default(),
        }
    }
}

pub struct GlobeBundleConfig {
    pub name: String,
}

impl Default for GlobeBundleConfig {
    fn default() -> Self {
        GlobeBundleConfig {
            name: String::from("planet_1")
        }
    }
}

pub fn setup_bundle(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>,
                    earth_constant: Res<EarthConstant>) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: earth_constant.x,
            ..Default::default()
        })),
        material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.6).into()),
        transform: Transform::default(),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(earth_constant.x + 100.0, earth_constant.y + 100.0, earth_constant.z + 100.0),
        ..Default::default()
    });
}

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bundle);
    }
}