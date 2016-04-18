use files;

pub struct ObjectData
{
    pub verts: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 3]>,
    pub faces: Vec<u8>,
}

fn parse_coord_list(coord_strs: &Vec<&str>) -> [f32; 3]
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

pub fn load_obj_file(path :&String) -> ObjectData
{
    let mut obj_data = ObjectData{verts: vec!(), normals: vec!(), uvs: vec!(), faces: vec!()};

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


        //let rest_of_line = segments[1..];
        let mut rest_of_line: Vec<&str> = Vec::new();
        rest_of_line.clone_from_slice(&(segments[1..]));

        match segments[0]
        {
            "v" => obj_data.verts.push(parse_coord_list(&rest_of_line)),
            "vn" => obj_data.verts.push(parse_coord_list(&rest_of_line)),
            _ => println!("Unknown obj file part: {} when loading {}", segments[0], path)
        }
    }

    return obj_data;
}
