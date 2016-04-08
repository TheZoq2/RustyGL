extern crate nalgebra as na;

use glium;
use glium::uniforms::UniformValue;

const MAX_SPHERICAL_LIGHTS: u32 = 32;

#[derive(Copy, Clone)]
struct SphericalLight
{
    position: na::Vec3<f32>,
    color: na::Vec3<f32>,
    range: f32
}

struct LightUniform
{
    sphere_light_count: u32,
    sphere_lights: [SphericalLight; MAX_SPHERICAL_LIGHTS as usize]
}

impl glium::uniforms::Uniforms for LightUniform
{
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("light_count", UniformValue::UnsignedInt(self.sphere_light_count));

        for i in 0..MAX_SPHERICAL_LIGHTS
        {
            f(&format!(
                    "spherical_light[{}].position", i), 
                    UniformValue::Vec3(self.sphere_lights[i as usize].position.as_ref().clone())
                );
            f(&format!(
                    "spherical_light[{}].color", i), 
                    UniformValue::Vec3(self.sphere_lights[i as usize].color.as_ref().clone())
                );
            f(&format!(
                    "spherical_light[{}].range", i), 
                    UniformValue::Float(self.sphere_lights[i as usize].range)
                );
        }
    }
}
