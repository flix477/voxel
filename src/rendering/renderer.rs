use crate::entities::{camera::Camera, Entity};
use crate::math::matrix::{get_projection_matrix, Matrix};
use crate::rendering::{entity_renderer::EntityRenderer, shaders::default::DEFAULT_SHADER};
use glium::{draw_parameters::DepthTest, Depth, Display, DrawParameters, Frame, Program, Surface};

const FOV: f32 = 70.0;
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 1024.0;

pub struct Renderer {
    program: Program,
    projection_matrix: Option<Matrix>,
    light: [f32; 3],
}

impl Renderer {
    pub fn new(display: &Display) -> Self {
        let program = Program::from_source(
            display,
            DEFAULT_SHADER.vertex,
            DEFAULT_SHADER.fragment,
            None,
        )
        .unwrap();

        Self {
            program,
            projection_matrix: None,
            light: [-1.0, 0.4, 0.9],
        }
    }

    pub fn render(&mut self, display: &mut Display, entities: &[Entity], camera: &Camera) {
        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.4, 1.0, 1.0), 1.0);

        if self.projection_matrix.is_none() {
            self.initialize_projection_matrix(&target);
        }

        let projection_matrix = self.projection_matrix.as_ref().unwrap();
        let camera_matrix = Matrix::camera_matrix(&camera.position, &camera.rotation).transposed();
        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        for entity in entities {
            EntityRenderer::render_entity(
                &self.program,
                &mut target,
                entity,
                projection_matrix.clone(),
                camera_matrix.clone(),
                &params,
                self.light.clone(),
            );
        }

        target.finish().unwrap()
    }

    fn initialize_projection_matrix(&mut self, target: &Frame) {
        let (width, height) = target.get_dimensions();
        self.projection_matrix = Some(get_projection_matrix(
            FOV,
            NEAR_PLANE,
            FAR_PLANE,
            height as f32 / width as f32,
        ));
    }
}
