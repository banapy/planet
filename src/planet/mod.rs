pub mod globe;
pub mod object;
pub mod camera;
pub mod atmosphere;
pub mod constant;
pub mod helper;
pub mod shape;

use bevy::prelude::*;
use std::default::{Default};
use atmosphere::*;
use constant::ConstantPlugin;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "planet".to_string(),
                width: 500.,
                height: 300.,
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            .add_plugins(DefaultPlugins)
            .add_plugin(helper::AxisHelperPlugin { length: 6378137.0 + 1000. })
            .add_plugin(ConstantPlugin)
            .add_plugin(globe::GlobePlugin)
            .add_plugin(camera::CameraPlugin)
            .run();
    }
}