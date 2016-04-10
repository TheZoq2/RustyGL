extern crate nalgebra as na;

use glium;
use glium::uniforms::UniformValue;

const MAX_SPHERICAL_LIGHTS: u32 = 32;


#[derive(Copy, Clone)]
pub struct SphericalLight
{
    position: [f32; 3],
    color: [f32; 3],
    range: f32
}
implement_uniform_block!(SphericalLight, position, color, range);

impl SphericalLight
{
    pub fn new() -> SphericalLight
    {
        SphericalLight
        {
            position: [0.0; 3],
            color: [0.0; 3],
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
implement_uniform_block!(LightUniform, sphere_light_count, sphere_lights);

impl glium::uniforms::Uniforms for LightUniform
{
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("sphere_light_count", UniformValue::UnsignedInt(self.sphere_light_count));

        for i in 0..MAX_SPHERICAL_LIGHTS
        {
            f(&format!(
                    "sphere_lights[{}].position", i), 
                    UniformValue::Vec3(self.sphere_lights[i as usize].position.clone())
                );
            f(&format!(
                    "sphere_lights[{}].color", i), 
                    UniformValue::Vec3(self.sphere_lights[i as usize].color.clone())
                );
            f(&format!(
                    "sphere_lights[{}].range", i), 
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
