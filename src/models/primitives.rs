use super::Vertex;

pub const QUAD_VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
    },
];

pub const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
