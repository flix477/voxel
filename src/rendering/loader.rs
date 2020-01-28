use crate::models::{Normal, RawModel, Vertex};
use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

pub struct Loader;
impl Loader {
    pub fn create_model(
        vertices: &[Vertex],
        indices: &[u16],
        normals: &[Normal],
        display: &Display,
    ) -> RawModel {
        let vertices = VertexBuffer::new(display, vertices).unwrap();
        let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap();
        let normals = VertexBuffer::new(display, normals).unwrap();
        RawModel {
            vertices,
            indices,
            normals,
        }
    }
}
