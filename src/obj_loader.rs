extern crate nalgebra as na;

use glium;
use files;

struct ObjectData
{
    pub verts: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 3]>,
    pub faces: Vec<u8>,
}

fn parse_coord_list(coord_strs: Vec<&str>)
{
    const COORD_AMOUNT :usize = 3;
    let curr_index = 0;
    let coords: [f32; COORD_AMOUNT] = [0.0; COORD_AMOUNT];
    //Iterate over all the coordinates in the vertex and add them to the object data
    for coord_str in coord_strs
    {
        if(curr_index < COORD_AMOUNT)
        {
            coords[curr_index] = coord_str.parse::<f32>().unwrap()
        }
        curr_index += 1;
    }
}
fn handle_vertex(coord_strs: Vec<&str>, &mut obj_data: ObjectData)
{
}

pub fn load_obj_file(path :&String)
{
    let mut object_data = ObjectData{verts: vec!(), normals: vec!(), uvs: vec!(), faces: vec!()};

    let file_content: String = files::load_whole_file(path);

    //Split the file  into individual lines
    let file_lines: Vec<&str>= file_content.split("\n").collect();

    for line in file_lines
    {
        
    }
}
