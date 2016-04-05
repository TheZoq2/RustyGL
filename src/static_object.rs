use model_data;
use glium;

pub struct StaticObject
{
    pub verts: Vec<model_data::Vertex>,
    pub normals: Vec<model_data::Normal>,
    pub indices: Vec<u16>,

    pub vertex_buffer: glium::VertexBuffer<model_data::Vertex>,
    pub normal_buffer: glium::VertexBuffer<model_data::Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

impl StaticObject
{
    /*
     *
     */
    fn new(mut display: glium::Display, verts: &Vec<model_data::Vertex>, normals: &Vec<model_data::Normal>, indices: &Vec<u16>) -> StaticObject
    {
        StaticObject{
            verts: verts.clone(),
            normals: normals.clone(),
            indices: indices.clone(),

            vertex_buffer: glium::VertexBuffer::new(&display, verts).unwrap(),
            normal_buffer: glium::VertexBuffer::new(&display, normals).unwrap(),
            index_buffer: glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, indices).unwrap()
        }
    }

    fn draw(&self, target: &glium::Frame, 
                shader_program: &glium::Program, 
                uniform_block: &glium::uniforms::UniformBlock, 
                draw_parameters: &glium::DrawParameters)
    {
        let uniforms = uniform!{global_params: uniform_block};

        target.draw((&self.vertex_buffer, &self.normal_buffer), 
                        &self.indices, 
                        shader_program, 
                        &uniforms, 
                        draw_parameters).unwrap();
    }
}


