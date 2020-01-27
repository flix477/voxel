pub mod primitives;

use glium::{implement_vertex, IndexBuffer, VertexBuffer};

pub struct RawModel {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);
