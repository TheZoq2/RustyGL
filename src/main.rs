#![allow(dead_code)]
#[macro_use]


extern crate glium;
extern crate nalgebra as na;

mod model_data;
mod static_object;
mod global_render_params;
mod lights;

use global_render_params::GlobalRenderParams;

use glium::DisplayBuild;
use glium::Surface;
use std::error::Error;
use std::io::prelude::*;


const VERTS: [model_data::Vertex; 6] = [
    model_data::Vertex{ position: ( 1.0,  0.0,  0.0) },
    model_data::Vertex{ position: ( 0.0,  1.0,  0.0) },
    model_data::Vertex{ position: ( 0.0,  0.0,  1.0) },
    model_data::Vertex{ position: (-1.0,  0.0,  0.0) },
    model_data::Vertex{ position: ( 0.0, -1.0,  0.0) },
    model_data::Vertex{ position: ( 0.0,  0.0, -1.0) },
];
const NORMALS: [model_data::Normal; 6] = [
    model_data::Normal{ normal: ( 1.0,  0.0,  0.0) },
    model_data::Normal{ normal: ( 0.0,  1.0,  0.0) },
    model_data::Normal{ normal: ( 0.0,  0.0,  1.0) },
    model_data::Normal{ normal: (-1.0,  0.0,  0.0) },
    model_data::Normal{ normal: ( 0.0, -1.0,  0.0) },
    model_data::Normal{ normal: ( 0.0,  0.0, -1.0) },
];

const INDICES: [u16; 24] = [
    0,1,2,
    0,2,4,
    0,4,5,
    0,5,1,
    3,1,2,
    3,2,4,
    3,4,5,
    3,5,1,
];

fn get_view_matrix(position: &[f32; 3], direction: &[f32; 3]) -> [[f32; 4]; 4] 
{
    //Make y axis up
    let up = [0.0, 1.0, 0.0];

    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();

        //Final value
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        
        //final value
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}


fn get_basic_shader(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::Program
{
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 modelMatrix;

        uniform worldData
        {
            mat4 view_matrix;
            mat4 projection_matrix;
        };

        out vec3 vertex_color;

        vec4 pos;

        void main()
        {
            mat4 modelViewMatrix = view_matrix * modelMatrix;
            pos = projection_matrix * modelViewMatrix * (vec4(position, 1.0));

            vertex_color = (position.xyz + vec3(1, 1, 1)) / 2;

            gl_Position = pos;
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec3 position;

        in vec3 vertex_color;

        out vec4 color;

        void main()
        {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    //Set up the shader
    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

fn load_whole_file(path: &String) -> String
{
    let file_path = std::path::Path::new(&path);

    let mut file = match std::fs::File::open(&file_path){
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
fn load_shader(display: &glium::Display, vert_path: &String, frag_path: &String)
    -> glium::Program
{
    let vertex_shader_src = load_whole_file(vert_path);
    let fragment_shader_src = load_whole_file(frag_path);

    //Set up the shader
    glium::Program::from_source(display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap()
}

fn get_perspective_matrix(target: &glium::Frame) -> [[f32; 4]; 4]
{
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov :f32 = 3.14 / 2.0;
    let zfar :f32 = 4096.0;
    let znear :f32 = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0]
    ]
}

fn main() 
{
    let mut t: f32 = 0.0;

    let display = glium::glutin::WindowBuilder::new()
                            .with_depth_buffer(24)
                            .build_glium()
                            .unwrap();

    let test_verts = vec!{
            model_data::Vertex{position: ( 1.0,  0.0,  0.0)},
            model_data::Vertex{position: ( 0.0,  1.0,  0.0)},
            model_data::Vertex{position: ( 0.0,  0.0,  1.0)},
            model_data::Vertex{position: (-1.0,  0.0,  0.0)},
            model_data::Vertex{position: ( 0.0, -1.0,  0.0)},
            model_data::Vertex{position: ( 0.0,  0.0, -1.0)}
        };
    let test_normals = vec!{
            model_data::Normal{normal: ( 1.0,  0.0,  0.0)},
            model_data::Normal{normal: ( 0.0,  1.0,  0.0)},
            model_data::Normal{normal: ( 0.0,  0.0,  1.0)},
            model_data::Normal{normal: ( 0.0,  0.0,  0.0)},
            model_data::Normal{normal: (-1.0,  0.0,  0.0)},
            model_data::Normal{normal: ( 0.0, -1.0,  0.0)},
            model_data::Normal{normal: ( 0.0,  0.0, -1.0)}
        };

    let test_indices = vec!{
        0,1,2,
        0,2,4,
        0,4,5,
        0,5,1,
        3,1,2,
        3,2,4,
        3,4,5,
        3,5,1,
    };
    let mut test_object = static_object::StaticObject::new(&display, &test_verts, &test_normals, &test_indices);

    let mut model_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-2.0, 0.0, 0.0, 1.0f32],
    ];


    //let shader_program = get_basic_shader(&display);
    let shader_program = load_shader(&display, &"data/shaders/basic.vert".to_string(), &"data/shaders/basic.frag".to_string());


    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let light = lights::LightUniform::new();

    loop
    {
        t += 0.01;

        //Rotate the model
        model_matrix[0][0] = t.cos();
        model_matrix[1][1] = t.cos();
        model_matrix[0][1] = t.sin();
        model_matrix[1][0] = -t.sin();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0 ,0.0 ,1.0), 1.0);

        //Creating the perspective matrix
        let perspective = get_perspective_matrix(&target);

        let view_matrix = get_view_matrix(&[0.0, 0.0, 4.0], &[0.0, 0.0, -1.0]);


        let vertex_buffer = glium::VertexBuffer::new(&display, &VERTS).unwrap();
        let normal_buffer = glium::VertexBuffer::new(&display, &NORMALS).unwrap();
        //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &INDICES).unwrap();

        let light_buffer = glium::uniforms::UniformBuffer::new(&display, light).unwrap();
        let sphere_buffer = glium::uniforms::UniformBuffer::new(&display, lights::SphericalLight::new()).unwrap();

        let render_params = GlobalRenderParams{ view_matrix: view_matrix, projection_matrix: perspective };
        let world_buffer = glium::uniforms::UniformBuffer::new(&display, render_params).unwrap();

        
        let uniforms = uniform!{
                modelMatrix: model_matrix,
                perspective: perspective,
                viewMatrix: view_matrix,

                worldData: &world_buffer,
                lights: &light_buffer,
                sphere: &sphere_buffer,
            };

        target.draw((&vertex_buffer, &normal_buffer), &indices, &shader_program, &uniforms, &params).unwrap();

        test_object.set_position(&na::Vec4::new(t.cos(), 0.0, 0.0, 1.0));
        //test_object.draw(&mut target, &shader_program, &world_buffer, &light_buffer, &params);

        //Finish drawing and send the result off to the window
        target.finish().unwrap();

        for ev in display.poll_events()
        {
            match ev
            {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
