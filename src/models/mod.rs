pub mod primitives;

use glium::{implement_vertex, IndexBuffer, VertexBuffer};

pub struct RawModel {
    pub vertices: &'static [Vertex],
    pub indices: &'static [u16],
    pub normals: &'static [Normal],
}

pub struct Model {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
    pub normals: VertexBuffer<Normal>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coords);

#[derive(Copy, Clone, Debug, Default)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);
