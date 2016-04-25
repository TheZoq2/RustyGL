use files;

pub struct ObjectData
{
    pub verts: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 3]>,
    pub faces: Vec<u32>,
}

fn add_vertex(vert: [f32; 3], obj_data: &mut ObjectData)
{
    obj_data.verts.push(vert);
    obj_data.normals.push([0.0, 0.0, 0.0]);
}

fn parse_coord_line(coord_strs: &Vec<&str>) -> [f32; 3]
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
fn parse_face_line(face_strs: &Vec<&str>, obj_data: &mut ObjectData, normal_list: &Vec<[f32; 3]>)
{
    for face in face_strs
    {
        let face_params: Vec<&str> = face_strs.split("/").collect();

        if segments.len() == 3
        {
            let mut face_index = 0;
            let mut normal_index = 0;

            match face_params[0].parse::<u32>()
            {
                Ok(val) => face_index = val,
                Err(e) => panic!("Failed to load .obj file, malformed vertex index in face. {}\n{}", face_params[0], e)
            }
            match face_params[0].parse::<u32>()
            {
                Ok(val) => normal_index = val,
                Err(e) => panic!("Failed to load .obj file, malformed normal index in face. {}\n{}", face_params[0], e)
            }

            //We have loaded all the index data, add it to the model.
            let added_normal = normal_list[normal_index];

            obj_data.normals[added_normal] += added_normal;
        }
        else
        {
            panic!("Wrong amount of arguments in 'face' part of .obj file. Should be vertex/uv/normal");
        }
    }
}

pub fn load_obj_file(path :&String) -> ObjectData
{
    let mut obj_data = ObjectData{verts: vec!(), normals: vec!(), uvs: vec!(), faces: vec!()};
    let mut normal_list: Vec<[f32; 3]> = vec!();

    let file_content: String = files::load_whole_file(path);

    //Split the file  into individual lines
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    for line in file_lines
    {
        //split the line at spaces
        let segments: Vec<&str> = line.split(" ").collect();

        //Skipping empty lines or comments
        if segments.len() == 0
        {
            continue;
        }

        let first_char = segments[0].chars().nth(0);
        match first_char
        {
            Some(t) => if t == '#' {continue},
            None => continue
        }


        //let rest_of_line = segments[1..];==> No kernel 4.5.1-1-ARCH headers. You must install them to use DKMS!
        let mut rest_of_line: Vec<&str> = Vec::new();
        rest_of_line.clone_from_slice(&(segments[1..]));

        match segments[0]
        {
            "v" => obj_data.verts.push(parse_coord_line(&rest_of_line)),
            "vn" => normal_list.push(parse_coord_line(&rest_of_line)),
            "f" => parse_face_line(&rest_of_line, &mut obj_data),
            _ => println!("Unknown obj file part: {} when loading {}", segments[0], path)
        }
    }

    return obj_data;
}
