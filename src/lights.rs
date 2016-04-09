extern crate nalgebra as na;

use glium;
use glium::uniforms::UniformValue;

const MAX_SPHERICAL_LIGHTS: u32 = 32;


#[derive(Copy, Clone)]
pub struct SphericalLight
{
    position: na::Vec3<f32>,
    color: na::Vec3<f32>,
    range: f32
}

impl SphericalLight
{
    pub fn new() -> SphericalLight
    {
        SphericalLight
        {
            position: na::zero(),
            color: na::one(),
            range: 0.0
        }
    }
}

#[derive(Copy, Clone)]
pub struct LightUniform
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

impl LightUniform
{
    pub fn new() -> LightUniform
    {
        LightUniform
        {
            sphere_light_count: 0,
            sphere_lights: [SphericalLight::new(); MAX_SPHERICAL_LIGHTS as usize]
        }
    }

    pub fn add_light(&mut self, light: SphericalLight)
    {
        if self.sphere_lights.len() < MAX_SPHERICAL_LIGHTS as usize
        {
            self.sphere_lights[self.sphere_light_count as usize] = light;

            self.sphere_light_count += 1;
        }
        else
        {
            panic!("Too many lights have been added");
        }
    }
}
