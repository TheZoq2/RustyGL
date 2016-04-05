use model_data;
use glium;

//pub struct StaticObject
//{
//    pub verts: Vec<model_data::Vertex>,
//    pub normals: Vec<model_data::Normal>,
//    pub indices: Vec<u16>,
//
//    pub vertex_buffer: glium::VertexBuffer<model_data::Vertex>,
//    pub normal_buffer: glium::VertexBuffer<model_data::Vertex>,
//    pub index_buffer: glium::IndexBuffer<u16>,
//}
//
//impl StaticObject
//{
//    /*
//     *
//     */
//    fn new(mut display: glium::backend::Facade, verts: &Vec<model_data::Vertex>, normals: &Vec<model_data::Normal>, indices: &Vec<u16>) -> StaticObject
//    {
//        StaticObject{
//            verts: verts.clone(),
//            normals: normals.clone(),
//            indices: indices.clone(),
//
//            vertex_buffer: glium::VertexBuffer::new(&display, verts).unwrap(),
//            normal_buffer: glium::VertexBuffer::new(&display, normals).unwrap(),
//            index_buffer: glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, indices).unwrap()
//        }
//    }
//}
//

#[derive(Clone,Copy)]
struct SomeStruct
{
    test: f32
}
#[derive(Clone,Copy)]
struct DependingStruct
{
    test: SomeStruct
}

impl DependingStruct
{
    fn new(a: &SomeStruct) -> DependingStruct
    {
        //do something using a here
        
        let new_struct = SomeStruct{test: 5.0};
        DependingStruct{test: new_struct}
    }
}

struct B
{
    pub a: SomeStruct,

    pub b: DependingStruct
}

impl B
{
    fn new(param_in: SomeStruct) -> B
    {
        //I can't let something = param.clone() and then use that in both places
        //because 'something' would get moved inside the struct before being passed as a reference
        //to ::new
        
        let something = param_in.clone();
        B //A struct that contains two fields
        {
            a: something,
            
            //This doesn't work because a is "undefined" here
            b: DependingStruct::new(&something)
        }
    }
}
