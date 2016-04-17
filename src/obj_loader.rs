extern crate nalgebra as na;

use glium;

struct ObjectData
{
    verts: std::vec<f32>,
    normals: std::vec<f32>,
    uvs: std::vec<f32>
    faces: std::vec<[u8; 2]>,

}
