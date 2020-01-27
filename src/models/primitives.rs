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

pub const CUBE_VERTICES: [Vertex; 24] = [
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
];

pub const CUBE_INDICES: [u16; 36] = [
    0, 1, 3, 3, 1, 2, 4, 5, 7, 7, 5, 6, 8, 9, 11, 11, 9, 10, 12, 13, 15, 15, 13, 14, 16, 17, 19,
    19, 17, 18, 20, 21, 23, 23, 21, 22,
];
