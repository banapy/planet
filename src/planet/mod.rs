pub mod globe;
pub mod object;
pub mod camera;
pub mod atmosphere;

use bevy::prelude::*;
use std::default::{Default};
use atmosphere::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(AtmosphereMat::default()) // Default AtmosphereMat, we can edit it to simulate another planet
            .add_plugins(DefaultPlugins)
            .add_startup_system(globe::setup_bundle)

            .add_plugin(AtmospherePlugin {
                dynamic: true,
                sky_radius: 20.0,
            })
            .add_startup_system(setup_environment)
            .add_system(daylight_cycle)

            .add_startup_system(camera::setup_bundle)
            .add_system(camera::camera_system)
            .run();
    }
}

#[derive(Component)]
struct Sun;

// We can edit the SkyMaterial resource and it will be updated automatically, as long as AtmospherePlugin.dynamic is true
fn daylight_cycle(
    mut sky_mat: ResMut<AtmosphereMat>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    time: Res<Time>,
) {
    let mut pos = sky_mat.sun_position;
    let t = time.time_since_startup().as_millis() as f32 / 1500.0;
    pos.y = t.sin();
    pos.z = t.cos();
    sky_mat.sun_position = pos;

    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        light_trans.rotation = Quat::from_rotation_x(-pos.y.atan2(pos.z));
        directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
    }
}

// Simple environment
fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Our Sun
    commands
        .spawn_bundle(DirectionalLightBundle {
            ..Default::default()
        })
        .insert(Sun); // Marks the light as Sun

    // Simple cube just for reference
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.5))),
        ..Default::default()
    });
}
