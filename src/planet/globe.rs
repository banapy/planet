use bevy::prelude::{Transform as BevyTransform};
use bevy_utils::Uuid;
use bevy::prelude::{PerspectiveCameraBundle};

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
pub struct Transform {
    ///local transform
    pub transform: BevyTransform,
    ///local transform
    pub world_transform: BevyTransform,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}


#[derive(Bundle)]
pub struct GlobeBundle {
    pub name: NamedObj,
    pub transform: Transform,
}

impl GlobeBundle {
    pub fn new() -> Self {
        const config: GlobeBundleConfig = Default::default();
        GlobeBundle::from_config(config)
    }
    pub fn from_config(config: GlobeBundleConfig) -> Self {
        GlobeBundle {
            name: UniqueId::new(config.name),
            transform: Default::default(),
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