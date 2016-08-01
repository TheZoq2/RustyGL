//Create normal and vertex structures
#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: (f32, f32, f32),
}
implement_vertex!(Vertex, position);

impl Vertex
{
    pub fn from_array(array: [f32; 3]) -> Vertex 
    {
        Vertex{
            position: (array[0], array[1], array[2])
        }
    }
}

#[derive(Copy, Clone)]
pub struct Normal
{
    pub normal: (f32, f32, f32),
}
implement_vertex!(Normal, normal);

impl Normal
{
    pub fn from_array(array: [f32; 3]) -> Normal
    {
        Normal{
            normal: (array[0], array[1], array[2])
        }
    }
}
