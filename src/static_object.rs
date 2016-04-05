use model_data;
use glium;

pub struct StaticObject
{
    pub verts: Vec<model_data::Vertex>,
    pub normals: Vec<model_data::Normal>,
    pub indices: Vec<u16>,

    pub vertex_buffer: glium::VertexBuffer<model_data::Vertex>,
    pub normal_buffer: glium::VertexBuffer<model_data::Vertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

impl StaticObject
{
    /*
     *
     */
    fn new(mut display: glium::backend::Facade, verts: &Vec<model_data::Vertex>, normals: &Vec<model_data::Normal>, indices: &Vec<u16>) -> StaticObject
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
}

//struct SomeStruct
//{
//    pub some_value: T,
//
//    pub some_dependent_value: TypeThatRequiresTInstance
//}
//
//impl SomeStruct
//{
//    fn new(a_value: T)
//    {
//        SomeStruct
//        {
//            some_value: a_value.clone(),
//    
//            some_dependent_value: TypeThatRequiresTInstance::new(&some_value))
//        }
//    }
//}
