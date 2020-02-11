use super::{Normal, RawModel, Vertex};

const CUBE_VERTICES: [Vertex; 24] = [
    // front
    Vertex {
        position: [-1.0, 1.0, -1.0],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        texture_coords: [1.0, 1.0],
    },
    // back
    Vertex {
        position: [-1.0, 1.0, 1.0],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        texture_coords: [1.0, 1.0],
    },
    // right
    Vertex {
        position: [1.0, 1.0, -1.0],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        texture_coords: [1.0, 1.0],
    },
    // left
    Vertex {
        position: [-1.0, 1.0, -1.0],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        texture_coords: [0.0, 1.0],
    },
    // top
    Vertex {
        position: [-1.0, 1.0, 1.0],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        texture_coords: [1.0, 1.0],
    },
    // bottom
    Vertex {
        position: [-1.0, -1.0, 1.0],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        texture_coords: [1.0, 0.0],
    },
];

const CUBE_INDICES: [u16; 36] = [
    0, 1, 3, 3, 1, 2, 4, 5, 7, 7, 5, 6, 8, 9, 11, 11, 9, 10, 12, 13, 15, 15, 13, 14, 16, 17, 19,
    19, 17, 18, 20, 21, 23, 23, 21, 22,
];

const CUBE_NORMALS: [Normal; 24] = [
    Normal {
        normal: [0.0, 0.0, -1.0],
    },
    Normal {
        normal: [0.0, 0.0, -1.0],
    },
    Normal {
        normal: [0.0, 0.0, -1.0],
    },
    Normal {
        normal: [0.0, 0.0, -1.0],
    },
    Normal {
        normal: [0.0, 0.0, 1.0],
    },
    Normal {
        normal: [0.0, 0.0, 1.0],
    },
    Normal {
        normal: [0.0, 0.0, 1.0],
    },
    Normal {
        normal: [0.0, 0.0, 1.0],
    },
    Normal {
        normal: [1.0, 0.0, 0.0],
    },
    Normal {
        normal: [1.0, 0.0, 0.0],
    },
    Normal {
        normal: [1.0, 0.0, 0.0],
    },
    Normal {
        normal: [1.0, 0.0, 0.0],
    },
    Normal {
        normal: [-1.0, 0.0, 0.0],
    },
    Normal {
        normal: [-1.0, 0.0, 0.0],
    },
    Normal {
        normal: [-1.0, 0.0, 0.0],
    },
    Normal {
        normal: [-1.0, 0.0, 0.0],
    },
    Normal {
        normal: [0.0, 1.0, 0.0],
    },
    Normal {
        normal: [0.0, 1.0, 0.0],
    },
    Normal {
        normal: [0.0, 1.0, 0.0],
    },
    Normal {
        normal: [0.0, 1.0, 0.0],
    },
    Normal {
        normal: [0.0, -1.0, 0.0],
    },
    Normal {
        normal: [0.0, -1.0, 0.0],
    },
    Normal {
        normal: [0.0, -1.0, 0.0],
    },
    Normal {
        normal: [0.0, -1.0, 0.0],
    },
];

pub const CUBE: RawModel = RawModel {
    vertices: &CUBE_VERTICES,
    indices: &CUBE_INDICES,
    normals: &CUBE_NORMALS,
};
