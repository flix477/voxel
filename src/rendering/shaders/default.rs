use crate::rendering::shaders::Shaders;

pub const DEFAULT_SHADER: Shaders = Shaders {
    vertex: VERTEX_SHADER,
    fragment: FRAGMENT_SHADER,
};

const VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 transformation_matrix;
    uniform mat4 projection_matrix;
    uniform mat4 camera_matrix;

    void main() {
        v_normal = transpose(inverse(mat3(transformation_matrix))) * normal;
        gl_Position = projection_matrix * camera_matrix * transformation_matrix * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 150

    in vec3 v_normal;

    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 entity_color;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = entity_color * 0.5;
        color = vec4(mix(dark_color, entity_color, brightness), 1.0);
    }
"#;
