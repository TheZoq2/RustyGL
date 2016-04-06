#[derive(Clone,Copy)]
pub struct GlobalRenderParams
{
    pub view_matrix: [[f32;4];4],
    pub projection_matrix: [[f32;4];4]
}
implement_uniform_block!(GlobalRenderParams, view_matrix, projection_matrix);
