use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

pub struct Line {
    pub length: f32,
    pub width: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            length: 1.,
            width: 1.,
        }
    }
}

impl From<Line> for Mesh {
    fn from(line: Line) -> Self {
        let vertices = vec![
            // [0.0, 0.0, 0.0],
            // [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 2.0],
        ];
        let uvs = vec![
            [0., 0.],
            [1., 1.],
            // [0., 0.],
            // [1., 1.],
        ];
        let indices = vec![ 2, 3];
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}