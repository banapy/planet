use bevy::math::{Vec3};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

//generate uuid.https://www.uuidtools.com/generate/v4
#[derive(Debug, TypeUuid)]
#[uuid = "d900ceb5-d5fd-4f48-b32d-99eb668af08b"]
pub struct CameraConstant {
    pub far: f32,
    pub near: f32,
    pub aspect_ratio: f32,
    pub fov: f32,
}

impl Default for CameraConstant {
    fn default() -> Self {
        Self {
            far: 10000000000.0,
            near: 0.1,
            aspect_ratio: 1.0,
            fov: std::f32::consts::PI / 4.0,
        }
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "381d4081-8bac-4116-a73c-4f5409ab3cc1"]
pub struct EarthConstant {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radii: Vec3,
    pub radii_squared: Vec3,
    pub one_over_radii: Vec3,
    pub one_over_radii_squared: Vec3,
    pub camera: CameraConstant,
}

impl Default for EarthConstant {
    fn default() -> Self {
        let x = 6378137.0;
        let y = 6378137.0;
        let z = 6378137.0;
        Self {
            x,
            y,
            z,
            radii: Vec3::new(x, y, z),
            radii_squared: Vec3::new(x * x, y * y, z * z),
            one_over_radii: Vec3::new(1.0 / x, 1.0 / y, 1.0 / z),
            one_over_radii_squared: Vec3::new(1.0 / x * x, 1.0 / y * y, 1.0 / z * z),
            camera:CameraConstant::default()
        }
    }
}

impl EarthConstant {
    fn get_res(&self) -> &Self {
        self
    }
}
pub struct ConstantPlugin;
impl Plugin for ConstantPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EarthConstant>();
    }
}