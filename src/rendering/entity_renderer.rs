use crate::entities::Entity;
use crate::math::matrix::Matrix;
use glium::{
    uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    DrawParameters, Frame, Program, Surface,
};

pub struct EntityRenderer;

impl EntityRenderer {
    pub fn render_entity(
        program: &Program,
        target: &mut Frame,
        entity: &Entity,
        projection_matrix: Matrix,
        camera_matrix: Matrix,
        draw_parameters: &DrawParameters,
        light: [f32; 3],
    ) {
        // TODO: transpose in glsl? transpose everything in matrix.rs?
        let Matrix(transformation_matrix) =
            Matrix::transformation(&entity.position, &entity.rotation, entity.scale).transposed();
        let uniforms = uniform! {
            transformation_matrix: transformation_matrix,
            projection_matrix: projection_matrix.take(),
            camera_matrix: camera_matrix.take(),
            entity_color: entity.color.take(),
            u_light: light,
            texture_coords: entity.texture.as_ref().unwrap().as_ref().sampled()
                .magnify_filter(MagnifySamplerFilter::Nearest)
                .minify_filter(MinifySamplerFilter::Nearest)
        };

        target
            .draw(
                (&entity.model.vertices, &entity.model.normals),
                &entity.model.indices,
                &program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
    }
}
