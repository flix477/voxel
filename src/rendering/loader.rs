use crate::models::{RawModel, Vertex};
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

pub struct Loader;
impl Loader {
    pub fn create_model(vertices: &[Vertex], indices: &[u16], display: &Display) -> RawModel {
        let vertices = VertexBuffer::new(display, vertices).unwrap();
        let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap();
        RawModel { vertices, indices }
    }
}
