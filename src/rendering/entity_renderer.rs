use crate::entities::Entity;
use crate::math::matrix::Matrix;
use glium::{uniform, Frame, Program, Surface};

pub struct EntityRenderer;

impl EntityRenderer {
    pub fn render_entity(
        program: &Program,
        target: &mut Frame,
        entity: &Entity,
        projection_matrix: &Matrix,
        camera_matrix: Matrix,
    ) {
        // TODO: transpose in glsl? transpose everything in matrix.rs?
        let Matrix(transformation_matrix) =
            Matrix::transformation(&entity.position, &entity.rotation, entity.scale).transposed();
        let Matrix(projection_matrix) = projection_matrix.clone();
        let uniforms = uniform! {
            transformation_matrix: transformation_matrix,
            projection_matrix: projection_matrix,
            camera_matrix: camera_matrix.transposed().take(),
            entity_color: entity.color.take()
        };

        target
            .draw(
                &entity.model.vertices,
                &entity.model.indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
