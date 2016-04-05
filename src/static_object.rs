extern crate nalgebra as na;

use model_data;
use glium;
use global_render_params;

use glium::Surface;
use na::Row;

pub struct StaticObject
{
    pub verts: Vec<model_data::Vertex>,
    pub normals: Vec<model_data::Normal>,
    pub indices: Vec<u16>,

    pub vertex_buffer: glium::VertexBuffer<model_data::Vertex>,
    pub normal_buffer: glium::VertexBuffer<model_data::Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,

    pub transform: na::Mat4<f32>
}

impl StaticObject
{
    /*
     *
     */
    fn new(mut display: glium::Display, verts: &Vec<model_data::Vertex>, normals: &Vec<model_data::Normal>, indices: &Vec<u16>) -> StaticObject
    {
        //let indices_slice = indices.as_slice();
        let indices_slice: [u16; 6] = [0,1,2,3,4,5];

        StaticObject{
            verts: verts.clone(),
            normals: normals.clone(),
            indices: indices.clone(),

            vertex_buffer: glium::VertexBuffer::new(&display, verts).unwrap(),
            normal_buffer: glium::VertexBuffer::new(&display, normals).unwrap(),
            index_buffer: glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices_slice).unwrap(),

            transform: na::zero()
        }
    }

    fn draw(&self, target: &mut glium::Frame, 
                shader_program: &glium::Program, 
                uniform_block: &glium::uniforms::UniformBuffer<global_render_params::GlobalRenderParams>, 
                draw_parameters: &glium::DrawParameters)
    {
        let uniforms = uniform!{global_params: uniform_block, transform_matrix: self.transform.as_ref().clone()};

        target.draw((&self.vertex_buffer, &self.normal_buffer), 
                        //&self.indices, 
                        &self.index_buffer,
                        shader_program, 
                        &uniforms, 
                        draw_parameters).unwrap();
    }

    fn set_position(&mut self, pos: &na::Vec4<f32>)
    {
        //na::translate(&self.transform, offset);
        self.transform.set_row(4, pos.clone());
    }
}


