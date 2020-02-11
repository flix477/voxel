use crate::{entities::Entity, models::Model, resources};
use glium::Display;
use std::error::Error;
use std::rc::Rc;

pub struct BlockMetadata {
    texture_path: &'static str,
}

impl BlockMetadata {
    pub fn create_entity(
        &self,
        display: &Display,
        cube: Rc<Model>,
    ) -> Result<Entity, Box<dyn Error>> {
        let texture = Rc::new(resources::load_texture(self.texture_path, display)?);
        Ok(Entity::new_with_texture(cube, texture))
    }
}

pub const DIRT: BlockMetadata = BlockMetadata {
    texture_path: "../dirt.png",
};
