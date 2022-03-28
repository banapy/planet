use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use crate::planet::shape::{Line};
use std::f32::consts::FRAC_PI_2;

fn setup_bundle(mut lines: ResMut<DebugLines>, mut axis_helper_plugin: Res<AxisHelperPlugin>) {
    let start = Vec3::new(0., 0., 0.);
    let length = 1. * axis_helper_plugin.length;
    lines.line_colored(
        start.clone(),
        Vec3::new(length, 0., 0.),
        99999999999999.,
        Color::RED,
    );
    lines.line_colored(
        start.clone(),
        Vec3::new(0., length, 0.),
        99999999999999.,
        Color::GREEN,
    );
    lines.line_colored(
        start.clone(),
        Vec3::new(0., 0., length),
        99999999999999.,
        Color::BLUE,
    );
}
#[derive(Clone)]
pub struct AxisHelperPlugin {
    pub length: f32,
}
impl Plugin for AxisHelperPlugin {
    fn build(&self, app: &mut App) {
        //register current plugin as resource
        app.insert_resource(self.clone());
        app.add_plugin(DebugLinesPlugin::default());
        app.add_startup_system(setup_bundle);
    }
}