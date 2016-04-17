use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub fn load_whole_file(path: &String) -> String
{
    let file_path = Path::new(&path);

    let mut file = match File::open(&file_path){
        Err(why) => panic!("Failed to open file {}, {}", &path, Error::description(&why)),

        Ok(open_file) => open_file
    };

    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Err(why) => panic!("Failed to read content of file {}, {}", &path, Error::description(&why)),
        Ok(a) => a
    };

    result
}
