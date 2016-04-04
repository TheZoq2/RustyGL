use model_data;
use glium;

pub struct StaticObject
{
    pub verts: Vec<model_data::Vertex>,
    pub normals: Vec<model_data::Normal>,
    pub indices: Vec<u16>,

    //pub vertex_buffer: glium::VertexBuffer,
    //pub normal_buffer: glium::VertexBuffer,
    //pub index_buffer: glium::IndexBuffer,
}

impl StaticObject
{
    fn create(&mut self, verts: &Vec<model_data::Vertex>, normals: &Vec<model_data::Normal>, indices: &Vec<u16>)
    {
        self.verts = verts.clone();
        self.normals = normals.clone();
        self.indices = indices.clone();

        //self.vertex_buffer = glium::VertexBuffer::new(&display, &VERTS).unwrap();
        //self.normal_buffer = glium::VertexBuffer::new(&display, &NORMALS).unwrap();
        //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        //self.indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &INDICES).unwrap();
    }
}
