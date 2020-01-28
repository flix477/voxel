pub mod primitives;

use glium::{implement_vertex, IndexBuffer, VertexBuffer};

pub struct RawModel {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
    pub normals: VertexBuffer<Normal>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug, Default)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);
