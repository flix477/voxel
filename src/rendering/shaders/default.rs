use crate::rendering::shaders::Shaders;

pub const DEFAULT_SHADER: Shaders = Shaders {
    vertex: VERTEX_SHADER,
    fragment: FRAGMENT_SHADER,
};

const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;

    uniform mat4 transformation_matrix;
    uniform mat4 projection_matrix;
    uniform mat4 camera_matrix;

    void main() {
        gl_Position = projection_matrix * camera_matrix * transformation_matrix * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 150

    uniform vec3 entity_color;
    out vec4 color;

    void main() {
        color = vec4(entity_color, 1.0);
    }
"#;
