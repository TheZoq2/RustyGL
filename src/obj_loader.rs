extern crate nalgebra as na;

use glium;

struct ObjectData
{
    verts: std::vec<f32>,
    normals: std::vec<f32>,
    uvs: std::vec<f32>
    faces: std::vec<[u8; 2]>,
}

fn load_obj_file(path :&String)
{
    let file_content = files::load_file_path(path);
}
