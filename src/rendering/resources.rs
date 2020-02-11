use crate::models::{Model, RawModel};
use glium::{
    index::PrimitiveType,
    texture::{RawImage2d, Texture2d},
    Display, IndexBuffer, VertexBuffer,
};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn create_model(model: &RawModel, display: &Display) -> Model {
    let vertices = VertexBuffer::new(display, model.vertices).unwrap();
    let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, model.indices).unwrap();
    let normals = VertexBuffer::new(display, model.normals).unwrap();
    Model {
        vertices,
        indices,
        normals,
    }
}

fn load_image(path: &str) -> Result<RawImage2d<u8>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let image = image::load(reader, image::PNG)?.to_rgba();
    let dimensions = image.dimensions();
    Ok(RawImage2d::from_raw_rgba_reversed(
        &image.into_raw(),
        dimensions,
    ))
}

pub fn load_texture(path: &str, display: &Display) -> Result<Texture2d, Box<dyn Error>> {
    let image = load_image(path)?;
    Texture2d::new(display, image).map_err(|e| Box::new(e) as Box<dyn Error>)
}
