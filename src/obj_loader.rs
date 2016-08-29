extern crate nalgebra as na;

use std::collections::VecDeque;

use files;
use model_data;

pub struct ObjectData
{
    pub verts: Vec<model_data::Vertex>,
    pub normals: Vec<model_data::Normal>,
    pub uvs: Vec<[f32; 2]>,
    pub faces: Vec<u16>,
}

//The indexes of the 3 parts of the face in the file
pub struct ObjFacePart
{
    pub vert: u32,
    pub uv: u32,
    pub normal: u32,
}
//A face in the .obj file consists of 3 or more vert/uv/normal pairs
pub struct ObjFace
{
    pub parts: Vec<ObjFacePart>,
}

//TODO: Rewrite this to something more generic to avoid duplicated code
fn parse_coord_line_2d(coord_strs: &VecDeque<&str>) -> [f32; 2]
{
    const COORD_AMOUNT :usize = 2;
    let mut curr_index = 0;
    let mut coords: [f32; COORD_AMOUNT] = [0.0; COORD_AMOUNT];
    //Iterate over all the coordinates in the vertex and add them to the object data
    for coord_str in coord_strs
    {
        if curr_index < COORD_AMOUNT
        {
            match coord_str.parse::<f32>()
            {
                Ok(val) => coords[curr_index] = val,
                Err(e) => panic!("failed to load .obj file, malformed coordinates: {} \n{}", coord_str, e)
            }
        }
        curr_index += 1;
    }

    return coords;
}
fn parse_coord_line(coord_strs: &VecDeque<&str>) -> [f32; 3]
{
    const COORD_AMOUNT :usize = 3;
    let mut curr_index = 0;
    let mut coords: [f32; COORD_AMOUNT] = [0.0; COORD_AMOUNT];
    //Iterate over all the coordinates in the vertex and add them to the object data
    for coord_str in coord_strs
    {
        if curr_index < COORD_AMOUNT
        {
            match coord_str.parse::<f32>()
            {
                Ok(val) => coords[curr_index] = val,
                Err(e) => panic!("failed to load .obj file, malformed coordinates: {} \n{}", coord_str, e)
            }
        }
        curr_index += 1;
    }

    return coords;
}
fn parse_face_line(face_strs: &VecDeque<&str>) -> ObjFace
{
    let mut result = ObjFace{parts: Vec::new()};

    for face_str in face_strs
    {
        let face_params: Vec<&str> = face_str.split("/").collect();

        //Ensuring that we have the correct amount of parenthesies
        if face_params.len() == 3
        {
            let mut face_part = ObjFacePart{vert: 0, uv:0, normal:0};


            //First element is  
            match face_params[0].parse::<u32>()
            {
                Ok(val) => face_part.vert = val as u32,
                Err(e) => panic!("Failed to load .obj file, malformed vertex index in face. {}\n{}", face_params[0], e)
            }
            match face_params[1].parse::<u32>()
            {
                Ok(val) => face_part.uv = val as u32,
                Err(e) => panic!("Failed to load .obj file, malformed uv index in face. {}\n{}", face_params[0], e)
            }
            match face_params[2].parse::<u32>()
            {
                Ok(val) => face_part.normal = val as u32,
                Err(e) => panic!("Failed to load .obj file, malformed normal index in face. {}\n{}", face_params[0], e)
            }

            //OBJ files are 1 indexed for some reason
            face_part.vert -= 1;
            face_part.normal -= 1;
            face_part.uv -= 1;
            result.parts.push(face_part);
        }
        else
        {
            panic!("Wrong amount of arguments in 'face' part of .obj file. Should be vertex/uv/normal");
        }

    }

    result
}

/**
  Obj files support one normal for each vertex in a face. Which means a single vertex can have 
  more than one normal. Glium and probably opengl don't support this so we need to split vertecies with
  multiple normals/uv coordinates into two vertecies.

  For now, all vertecies are split to make things easier
 */
//TODO: Don't split vertecies whichc have the same normals/uvs
fn generate_valid_object(verts: Vec<[f32; 3]>, normals: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>, faces: Vec<ObjFace>) -> ObjectData
{
    let mut obj_data = ObjectData{verts: vec!(), normals: vec!(), uvs: vec!(), faces: vec!()};

    for face in faces
    {
        //For now all faces must be triangles
        assert!(face.parts.len() == 3);

        for point in face.parts
        {
            println!("{}", point.vert);
            let vertex = model_data::Vertex::from_array(verts[point.vert as usize]);
            let normal = model_data::Normal::from_array(normals[point.normal as usize]);
            let uv = uvs[point.uv as usize];

            //Add the point to the obj data
            obj_data.verts.push(vertex);
            obj_data.normals.push(normal);
            obj_data.uvs.push(uv);
            obj_data.faces.push(obj_data.verts.len() as u16 - 1);
        }
    }

    obj_data
}

pub fn load_obj_file(path :&String) -> ObjectData
{
    let mut vert_list = Vec::<[f32; 3]>::new();
    let mut normal_list = Vec::<[f32; 3]>::new();
    let mut uv_list = Vec::<[f32; 2]>::new();

    let mut face_list = Vec::<ObjFace>::new();

    let file_content: String = files::load_whole_file(path);

    //Split the file  into individual lines
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    //Parse all the lines in the file
    for line in file_lines
    {
        //split the line at spaces
        let mut segments: VecDeque<&str> = line.split(" ").collect();

        //Skipping empty lines
        if segments.len() == 0
        {
            continue;
        }

        //Skipping comments
        let first_char = segments[0].chars().nth(0);
        match first_char
        {
            Some(t) => if t == '#' {continue},
            None => continue
        }


        let mut rest_of_line: Vec<&str> = Vec::new();

        println!("Segment length: {}", segments.len());

        rest_of_line.clone_from_slice(&(segments[1..]));

        match segments[0]
        {
            "v" => vert_list.push(parse_coord_line(&segments)),
            "vn" => normal_list.push(parse_coord_line(&segments)),
            "vt" => uv_list.push(parse_coord_line_2d(&segments)),
            "f" => face_list.push(parse_face_line(&segments)),
            _ => println!("Unknown obj file part: {} when loading {}", first_word, path)
        }
    }

    generate_valid_object(vert_list, normal_list, uv_list, face_list)
}
